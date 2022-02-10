use crate::JsonMessage;
use futures::future::BoxFuture;
use google_cloud::pubsub::Message;
use serde::Deserialize;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::Mutex;

pub trait MessageHandler: Send {
    fn on_message(&self, message: Message) -> BoxFuture<'static, ()>;
}

pub struct JsonMessageHandler<T, F, Fut> {
    function: F,
    phantom_body: PhantomData<Mutex<T>>,
    phantom_future: PhantomData<Mutex<Fut>>,
}

impl<T, F, Fut> MessageHandler for JsonMessageHandler<T, F, Fut>
    where
        T: for<'de> Deserialize<'de> + Send,
        F: Fn(JsonMessage<T>) -> Fut + Send + 'static,
        Fut: Future<Output=()> + Send + 'static,
{
    fn on_message(&self, message: Message) -> BoxFuture<'static, ()> {
        let message = JsonMessage::try_from(message).expect("Failed to deserialize message");

        Box::pin((self.function)(message))
    }
}

impl<T, F, Fut> From<F> for JsonMessageHandler<T, F, Fut>
    where
        T: for<'de> Deserialize<'de> + Send,
        F: Fn(JsonMessage<T>) -> Fut + Send + 'static,
        Fut: Future<Output=()> + Send + 'static,
{
    fn from(function: F) -> Self {
        Self {
            function,
            phantom_body: PhantomData,
            phantom_future: PhantomData,
        }
    }
}


