use clap::Parser;
use gcp_pubsub_rust::acquire_topic;
use google_cloud::pubsub::Client;
use std::error::Error;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// Message to send
    message: String,

    /// Name of the topic to send a message to
    #[clap(short, long)]
    topic: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().unwrap();
    env_logger::init();

    let args: Args = Args::parse();

    let mut client = Client::new().await?;
    let mut topic = acquire_topic(&mut client, &args.topic).await?;
    let message = args.message.clone();
    topic.publish(message).await?;
    log::info!("Sent message '{}' to topic '{}'", args.message, args.topic);

    Ok(())
}
