use google_cloud::pubsub::Message;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

pub trait FromMessage: Sized {
    fn from_message(message: Message) -> Result<Self, Box<dyn std::error::Error>>;
}

impl FromMessage for Message {
    fn from_message(message: Message) -> Result<Self, Box<dyn Error>> {
        Ok(message)
    }
}

impl FromMessage for () {
    fn from_message(_message: Message) -> Result<Self, Box<dyn Error>> {
        Ok(())
    }
}

impl<F> FromMessage for (F,)
where
    F: FromMessage
{
    fn from_message(message: Message) -> Result<Self, Box<dyn Error>> {
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

    pub fn id(&self) -> &str {
        self.message.id()
    }

    pub fn data(&self) -> &[u8] {
        self.message.data()
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        self.message.attributes()
    }

    pub fn publish_time(&self) -> chrono::NaiveDateTime {
        self.message.publish_time()
    }

    pub async fn ack(&mut self) -> Result<(), google_cloud::error::Error> {
        self.message.ack().await
    }

    pub async fn nack(&mut self) -> Result<(), google_cloud::error::Error> {
        self.message.nack().await
    }
}

impl<T> FromMessage for JsonMessage<T>
where
    T: Sized + for<'de> Deserialize<'de>
{
    fn from_message(message: Message) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            body: serde_json::from_slice(message.data())?,
            message,
        })
    }
}
