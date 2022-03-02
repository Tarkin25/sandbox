use gcp_pubsub_rust::{Application, Handler, JsonMessage, MessageFacade, ProcessingResult};
use serde::Deserialize;
use std::error::Error;
use std::future::Future;
use futures::future::BoxFuture;
use google_cloud::pubsub::Message;
use tokio::signal;

#[derive(Deserialize)]
struct TestMessage {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().unwrap();
    env_logger::init();

    async fn handle_subscription(message: JsonMessage<TestMessage>) -> ProcessingResult {
        message.ack().await
    }

    let app = Application::new()
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
