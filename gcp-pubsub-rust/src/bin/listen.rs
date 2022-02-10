use gcp_pubsub_rust::{Application, JsonMessage, Listen};
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

    let app = Application::new()
        .listen(
            "test-topic",
            "test-subscription",
            |mut message: JsonMessage<TestMessage>| async move {
                message.ack().await.unwrap();
            }
        )
        .start()
        .await?;

    signal::ctrl_c().await?;

    app.stop().await?;

    Ok(())
}
