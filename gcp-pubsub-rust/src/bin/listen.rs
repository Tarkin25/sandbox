use gcp_pubsub_rust::{Application, JsonMessage};
use serde::Deserialize;
use std::error::Error;
use tokio::signal;

#[derive(Deserialize)]
struct TestMessage {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().unwrap();
    env_logger::init();

    async fn handle_subscription(mut message: JsonMessage<TestMessage>) {
        message.ack().await.unwrap();
    }

    let app = Application::new()
        .listen(
            "test-topic",
            "test-subscription",
            |mut message: JsonMessage<TestMessage>| async move {
                message.ack().await.unwrap();
            }
        )
        .listen(
            "test-topic",
            "test-subscription",
            || async {
                println!("New message received");
            }
        )
        .listen(
            "test-topic",
            "test-subscription",
            handle_subscription
        )
        .start()
        .await?;

    signal::ctrl_c().await?;

    app.stop().await?;

    Ok(())
}
