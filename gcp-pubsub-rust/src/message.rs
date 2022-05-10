use std::any::type_name;
use google_cloud::pubsub::Message;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use anyhow::Context;

#[async_trait]
pub trait MessageFacade {
    fn message(&self) -> &Message;

    fn message_mut(&mut self) -> &mut Message;

    fn id(&self) -> &str {
        self.message().id()
    }

    fn data(&self) -> &[u8] {
        self.message().data()
    }

    fn attributes(&self) -> &HashMap<String, String> {
        self.message().attributes()
    }

    fn publish_time(&self) -> chrono::NaiveDateTime {
        self.message().publish_time()
    }

    async fn ack(mut self) -> ProcessingSignal where Self: Sized {
        ProcessingSignal(self.message_mut().ack().await)
    }

    async fn nack(mut self) -> ProcessingSignal where Self: Sized {
        ProcessingSignal(self.message_mut().nack().await)
    }
}

pub struct ProcessingSignal(pub(crate) Result<(), google_cloud::error::Error>);

pub trait FromMessage: Sized {
    fn from_message(message: Message) -> anyhow::Result<Self>;
}

impl FromMessage for Message {
    fn from_message(message: Message) -> anyhow::Result<Self> {
        Ok(message)
    }
}

impl FromMessage for () {
    fn from_message(_message: Message) -> anyhow::Result<Self> {
        Ok(())
    }
}

impl<F> FromMessage for (F,)
where
    F: FromMessage
{
    fn from_message(message: Message) -> anyhow::Result<Self> {
        Ok((F::from_message(message)?,))
    }
}

pub struct JsonMessage<T>
{
    message: Message,
    body: T,
}

impl<T> JsonMessage<T>
{
    pub fn body(&self) -> &T {
        &self.body
    }

    pub fn into_body(self) -> T {
        self.body
    }
}

impl<T> MessageFacade for JsonMessage<T> {
    fn message(&self) -> &Message {
        &self.message
    }

    fn message_mut(&mut self) -> &mut Message {
        &mut self.message
    }
}

impl<T> FromMessage for JsonMessage<T>
where
    T: Sized + for<'de> Deserialize<'de>
{
    fn from_message(message: Message) -> anyhow::Result<Self> {
        Ok(Self {
            body: serde_json::from_slice(message.data()).context(format!("Unable to deserialize JsonMessage<{}>", type_name::<T>()))?,
            message,
        })
    }
}
