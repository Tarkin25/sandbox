use std::process;
use std::time::Duration;

use chrono::{Utc};

use connection_tracker::{ FailureLogger, HttpSingleProbe, ConnectionChecker, FailureTracker };

const CSV_FILE_NAME: &str = "connection-tracker.csv";
const CONNECTION_TIMEOUT: u64 = 5;
const PROBE_INTERVAL: u64 = 5;

#[tokio::main]
async fn main() {

    let failure_logger = match FailureLogger::new(CSV_FILE_NAME) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Unable to initialize failure logger: {}", e);
            process::exit(1);
        }
    };

    let probe = HttpSingleProbe::new("https://google.com/search", CONNECTION_TIMEOUT);

    let tracker = FailureTracker::new();

    let handle_connection_failure = || {
        tracker.failure_start(Utc::now());

        println!("Connection failed: Reconnecting...");
    };

    let handle_reconnect = || {
        tracker.failure_end(Utc::now());

        println!("Reconnected.");

        let connection_failure = tracker.current_failure().unwrap();

        if let Err(e) = failure_logger.log(&connection_failure) {
            eprintln!("Unable to persist {:?}: {}", connection_failure, e);
        }
    };

    let mut checker = ConnectionChecker::builder()
        .probe(probe)
        .probe_interval(Duration::from_secs(PROBE_INTERVAL))
        .on_connection_failure(handle_connection_failure)
        .on_reconnect(handle_reconnect)
        .build();

    checker.start().await;
}