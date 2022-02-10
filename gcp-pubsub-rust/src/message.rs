use google_cloud::pubsub::Message;
use serde::Deserialize;
use std::collections::HashMap;

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

impl<T> TryFrom<Message> for JsonMessage<T>
where
    T: for<'de> Deserialize<'de>,
{
    type Error = serde_json::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        Ok(Self {
            body: serde_json::from_slice(message.data())?,
            message,
        })
    }
}
