use google_cloud::pubsub::{Client, Subscription, SubscriptionConfig, Topic, TopicConfig};

pub use crate::handler::{Handler, HandlerFuture};
pub use application::*;
pub use message::*;

#[macro_use]
extern crate async_trait;

mod application;
mod handler;
mod message;

pub async fn acquire_topic(
    client: &mut Client,
    id: &str,
) -> Result<Topic, google_cloud::error::Error> {
    if let Ok(topic) = client.create_topic(id, TopicConfig::default()).await {
        Ok(topic)
    } else {
        client.topic(id).await.map(Option::unwrap)
    }
}

pub async fn acquire_subscription(
    client: &mut Client,
    topic: &mut Topic,
    id: &str,
) -> Result<Subscription, google_cloud::error::Error> {
    if let Ok(subscription) = topic
        .create_subscription(id, SubscriptionConfig::default())
        .await
    {
        Ok(subscription)
    } else {
        client.subscription(id).await.map(Option::unwrap)
    }
}
