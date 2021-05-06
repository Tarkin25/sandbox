use std::thread;
use std::process;
use std::time::Duration;

use chrono::Utc;
use reqwest::{Client, Request};

use connection_tracker::{ConnectionFailure, FailureLogger};

const CSV_FILE_NAME: &str = "connection-tracker.csv";
const CONNECTION_TIMEOUT: u64 = 10;
const PROBE_INTERVAL: u64 = 60;

#[tokio::main]
async fn main() {

    let client = Client::new();

    let failure_logger = match FailureLogger::new(CSV_FILE_NAME) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Unable to initialize failure logger: {}", e);
            process::exit(1);
        }
    };

    loop {
        test_connection(&client).await;

        let failure_start = Utc::now();

        println!("Connection failed. Reconnecting...");

        reconnect(&client).await;

        println!("Reconnected.");

        let failure_end = Utc::now();

        let connection_failure = ConnectionFailure::new(failure_start, failure_end);

        if let Err(e) = failure_logger.log(&connection_failure) {
            eprintln!("Unable to persist {:?}: {}", connection_failure, e);
        }
    }
}

async fn test_connection(client: &Client) {
    loop {
        let request = create_request(client);

        if let Ok(_) = client.execute(request).await {
            println!("Connection ok.");

            thread::sleep(Duration::from_secs(PROBE_INTERVAL));
        } else {
            break;
        }
    }
}

async fn reconnect(client: &Client) {
    loop {
        let request = create_request(client);

        if let Err(_) = client.execute(request).await {
            println!("Re-connect failed.");

            thread::sleep(Duration::from_secs(PROBE_INTERVAL));
        } else {
            break;
        }
    }
}

fn create_request(client: &Client) -> Request {
    client.get("https://google.com/search").timeout(Duration::from_secs(CONNECTION_TIMEOUT)).build().unwrap()
}