use crate::{acquire_subscription, acquire_topic, JsonMessage};
use futures::future::BoxFuture;
use google_cloud::pubsub::{Client, Message};
use serde::Deserialize;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinHandle;

struct Listener {
    topic: String,
    subscription: String,
    handler: Arc<dyn Fn(Message) -> BoxFuture<'static, ()> + Sync + Send>,
}

#[derive(Default)]
pub struct ApplicationBuilder {
    listeners: Vec<Listener>,
}

impl ApplicationBuilder {
    fn new() -> Self {
        Self { listeners: vec![] }
    }

    pub fn listen<T, F, Fut>(
        mut self,
        topic: impl Into<String>,
        subscription: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: Fn(JsonMessage<T>) -> Fut + Send + Sync + Copy + 'static,
        Fut: Future<Output = ()> + Send + 'static,
        T: for<'de> Deserialize<'de> + Send,
    {
        let handler = move |message: Message| {
            let message = JsonMessage::try_from(message).expect("Failed to deserialize message");

            handler(message)
        };

        self.listeners.push(Listener {
            topic: topic.into(),
            subscription: subscription.into(),
            handler: Arc::new(move |message| Box::pin(handler(message))),
        });

        self
    }

    pub async fn start(self) -> Result<Application, Box<dyn std::error::Error>> {
        Application::start(self).await
    }
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
        let mut client = Client::new().await?;

        let listeners = builder.listeners;
        let (tx, _rx) = broadcast::channel::<()>(listeners.len());

        let mut join_handles = vec![];

        for listener in listeners {
            let mut topic = acquire_topic(&mut client, &listener.topic).await?;
            let topic_id = topic.id().to_string();
            let mut subscription =
                acquire_subscription(&mut client, &mut topic, &listener.subscription).await?;
            let subscription_id = subscription.id().to_string();
            let mut rx = tx.subscribe();

            let handle = tokio::spawn(async move {
                loop {
                    let handler = Arc::clone(&listener.handler);

                    let message = tokio::select! {
                        message = subscription.receive() => Some(message),
                        _ = rx.recv() => None,
                    };

                    if let Some(message) = message {
                        if let Some(message) = message {
                            log::info!(target: topic.id(), "Received message: {}", String::from_utf8_lossy(message.data()));
                            handler(message).await;
                        }
                    } else {
                        break;
                    }
                }
            });

            join_handles.push(handle);

            log::info!(
                "Listening for messages on topic '{}' using subscription '{}'",
                topic_id,
                subscription_id
            );
        }

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
