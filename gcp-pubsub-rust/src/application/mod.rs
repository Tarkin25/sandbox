mod builder;

use anyhow::Context;
use crate::{acquire_subscription, acquire_topic, ProcessingSignal};
use google_cloud::pubsub::Client;
use futures::stream::FuturesUnordered;
use futures::{TryStreamExt};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinHandle;

pub use builder::*;

pub struct Application {
    client: Client,
    join_handles: Vec<JoinHandle<()>>,
    shutdown_sender: Sender<()>,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }

    async fn start(builder: ApplicationBuilder) -> anyhow::Result<Self> {
        let client = Client::new().await.context("Unable to create client")?;

        let listeners = builder.listeners;
        let (tx, _rx) = broadcast::channel::<()>(listeners.len());

        let tx_ref = &tx;
        let client_clone = client.clone();

        let join_handles = listeners.into_iter()
            .map(move |listener| create_subscription_handle(client_clone.clone(), listener, tx_ref))
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await.context("Unable to create subscription handles")?;

        Ok(Self {
            client,
            join_handles,
            shutdown_sender: tx,
        })
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        log::info!("Shutting down");
        self.shutdown_sender.send(()).context("Unable to send shutdown signal")?;

        for handle in self.join_handles {
            handle.await.context("Unable to await thread join handle")?;
        }

        Ok(())
    }
}

async fn create_subscription_handle(mut client: Client, listener: Listener, tx: &Sender<()>) -> anyhow::Result<JoinHandle<()>> {
    let mut topic = acquire_topic(&mut client, &listener.topic).await.context("Unable to acquire topic")?;
    let topic_id = topic.id().to_string();
    let mut subscription =
        acquire_subscription(&mut client, &mut topic, &listener.subscription).await.context("Unable to acquire subscription")?;
    let subscription_id = subscription.id().to_string();
    let mut rx = tx.subscribe();

    let handle = tokio::spawn(async move {
        loop {
            let message = tokio::select! {
                        message = subscription.receive() => Some(message),
                        _ = rx.recv() => None,
                    };

            if let Some(message) = message {
                if let Some(message) = message {
                    log::info!(target: topic.id(), "Received message: {}", String::from_utf8_lossy(message.data()));
                    let future = listener.handler.on_message(message);

                    let result = future.await;

                    match result {
                        Ok(ProcessingSignal(Err(signal_error))) => log::error!(target: topic.id(), "Error while ack-/nacking message: {}", signal_error),
                        Err(handle_error) => log::error!(target: topic.id(), "Error while handling message: {:?}", handle_error),
                        _ => {}
                    }
                }
            } else {
                break;
            }
        }
    });

    log::info!(
                "Listening for messages on topic '{}' using subscription '{}'",
                topic_id,
                subscription_id
            );

    Ok(handle)
}
