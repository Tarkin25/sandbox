use cloud_pubsub::{Client, EncodedMessage, FromPubSubMessage};
use cloud_pubsub::error::Error;
use dotenv::dotenv;
use hyper::{Body, Request};
use lazy_static::lazy_static;
use std::env;

#[macro_use]
extern crate async_trait;

lazy_static! {
    static ref PUBSUB_HOST: String = env::var("PUBSUB_EMULATOR_HOST")
        .map(|host| format!("http://{}", host))
        .unwrap_or_else(|_| String::from("https://pubsub.googleapis.com"));
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to initialize dotenv");
    env_logger::init();

    let mut pubsub = Client::new("local/gcp-service-account.json".into())
        .await
        .expect("Unable to create client");
    pubsub.set_project("test-project".into());

    let topic_name = String::from("test-topic");
    pubsub.create_topic(topic_name.clone()).await.expect("Failed to create topic 'test-topic'");
    let topic = pubsub.topic(topic_name.clone());

    let subscription = topic.subscribe()
        .await
        .expect("Failed to subscribe to topic 'test-topic'");

    topic.publish("This is a test message")
        .await
        .expect("Failed to publish message to topic 'test-topic'");

    let messages = subscription.get_messages::<TestMessage>()
        .await
        .expect("Failed to pull messages");

    for message in &messages {
        log::info!("Received message: {:#?}", message);
    }

    let ack_ids = messages.into_iter()
        .map(|(_, ack_id)| ack_id)
        .collect::<Vec<_>>();

    subscription.acknowledge_messages(ack_ids).await;

    log::info!("Cleaning up");

    subscription.destroy().await.expect("Failed to delete subscription");
}

#[derive(Debug)]
struct TestMessage(String);

impl FromPubSubMessage for TestMessage {
    fn from(message: EncodedMessage) -> Result<Self, Error> {
        match message.decode() {
            Ok(bytes) => Ok(TestMessage(String::from_utf8_lossy(&bytes).into_owned())),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
trait ClientExt {
    async fn create_topic(&self, name: String) -> hyper::Result<()>;
}

#[async_trait]
impl ClientExt for Client {
    async fn create_topic(&self, name: String) -> hyper::Result<()> {
        let request = Request::builder()
            .method("PUT")
            .uri(format!("{}/v1/projects/{}/topics/{}", *PUBSUB_HOST, self.project(), name))
            .body(Body::empty()).expect("Failed to create request");

        self.hyper_client()
            .request(request)
            .await?;

        Ok(())
    }
}


