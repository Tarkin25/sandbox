mod builder;

use crate::{acquire_subscription, acquire_topic};
use google_cloud::pubsub::Client;
use futures::stream::FuturesUnordered;
use futures::{TryStreamExt};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinHandle;
use crate::application::builder::ApplicationBuilder;
use crate::handler::{RawHandler};

pub use builder::*;

pub(crate) struct Listener {
    topic: String,
    subscription: String,
    handler: Box<dyn RawHandler>,
}

pub struct Application {
    client: Client,
    join_handles: Vec<JoinHandle<()>>,
    shutdown_sender: Sender<()>,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }

    async fn start(builder: ApplicationBuilder) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new().await?;

        let listeners = builder.listeners;
        let (tx, _rx) = broadcast::channel::<()>(listeners.len());

        let tx_ref = &tx;
        let client_clone = client.clone();

        let join_handles = listeners.into_iter()
            .map(move |listener| create_subscription_handle(client_clone.clone(), listener, tx_ref))
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await?;

        Ok(Self {
            client,
            join_handles,
            shutdown_sender: tx,
        })
    }

    pub async fn stop(self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Shutting down");
        self.shutdown_sender.send(())?;

        for handle in self.join_handles {
            handle.await?;
        }

        Ok(())
    }
}

async fn create_subscription_handle(mut client: Client, listener: Listener, tx: &Sender<()>) -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
    let mut topic = acquire_topic(&mut client, &listener.topic).await?;
    let topic_id = topic.id().to_string();
    let mut subscription =
        acquire_subscription(&mut client, &mut topic, &listener.subscription).await?;
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
                    future.await.0.expect("Error while processing message");
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
