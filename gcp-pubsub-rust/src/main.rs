use google_cloud::pubsub::{Client, SubscriptionConfig, TopicConfig};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    env_logger::init();

    let mut client = Client::new("test-project").await.unwrap();
    log::info!("Created client");

    let mut topic = if let Ok(topic) = client.create_topic("test-topic", TopicConfig::default()).await {
        topic
    } else {
        client.topic("test-topic").await.unwrap().unwrap()
    };
    log::info!("Got handle to topic");

    let mut subscription = if let Ok(subscription) = topic.create_subscription("test-subscription", SubscriptionConfig::default()).await {
        subscription
    } else {
        client.subscription("test-subscription").await.unwrap().unwrap()
    };
    log::info!("Got handle to subscription");

    #[derive(Serialize, Deserialize, Debug)]
    struct HelloWorld<'a> {
        hello: &'a str,
    }

    let message = HelloWorld { hello: "world" };
    let message = json::to_vec(&message).unwrap();
    topic.publish(message).await.unwrap();
    log::info!("Published message");

    let mut received = subscription.receive().await.unwrap();
    let message = json::from_slice::<HelloWorld>(received.data()).unwrap();
    log::info!("Received message: {:?}", message);

    received.ack().await.unwrap();
    log::info!("Acknowledged message");

    subscription.delete().await.unwrap();
    log::info!("Deleted subscription");

    topic.delete().await.unwrap();
    log::info!("Deleted topic");
}