use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;
use futures::future::BoxFuture;
use google_cloud::pubsub::Message;
use serde::Deserialize;
use tokio::sync::Mutex;
use crate::application::Listener;
use crate::{Application, JsonMessage};
use crate::handler::{JsonMessageHandler, MessageHandler};

#[derive(Default)]
pub struct ApplicationBuilder {
    pub(crate) listeners: Vec<Listener>,
}

impl ApplicationBuilder {
    pub(crate) fn new() -> Self {
        Self { listeners: vec![] }
    }

    pub async fn start(self) -> Result<Application, Box<dyn std::error::Error>> {
        Application::start(self).await
    }

    fn add_listener<T: MessageHandler + 'static>(mut self, topic: impl Into<String>, subscription: impl Into<String>, handler: T) -> Self {
        self.listeners.push(Listener {
            topic: topic.into(),
            subscription: subscription.into(),
            handler: Box::new(handler)
        });

        self
    }
}

pub trait Listen<T, F, Fut> {
    fn listen(self, topic: impl Into<String>, subscription: impl Into<String>, function: F) -> Self;
}

impl<T, F, Fut> Listen<T, F, Fut> for ApplicationBuilder
where
    T: for<'de> Deserialize<'de> + Send + 'static,
    F: Fn(JsonMessage<T>) -> Fut + Send + 'static,
    Fut: Future<Output=()> + Send + 'static,
{
    fn listen(self, topic: impl Into<String>, subscription: impl Into<String>, function: F) -> Self {
        self.add_listener(topic, subscription, JsonMessageHandler::from(function))
    }
}
