use gcp_pubsub_rust::{Application, Handler, JsonMessage, MessageFacade, ProcessingResult};
use serde::Deserialize;
use std::error::Error;
use futures::future::BoxFuture;
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

    struct MyHandler;

    impl Handler<JsonMessage<TestMessage>> for MyHandler {
        type Future = BoxFuture<'static, ProcessingResult>;

        fn call(&self, message: JsonMessage<TestMessage>) -> Self::Future {
            Box::pin(message.ack())
        }
    }

    let app = Application::new()
        .listen(
            "test-topic",
            "test-subscription",
            |message: JsonMessage<TestMessage>| async move {
                message.ack().await
            }
        )
        .listen(
            "test-topic",
            "test-subscription",
            handle_subscription
        )
        .listen(
            "test-topic",
            "test-subscription",
            MyHandler
        )
        .start()
        .await?;

    signal::ctrl_c().await?;

    app.stop().await?;

    Ok(())
}
