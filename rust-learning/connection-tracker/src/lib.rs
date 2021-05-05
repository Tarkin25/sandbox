use std::fmt::Debug;
use chrono::{DateTime, Utc, Duration};

mod log;

pub use log::*;

#[derive(Debug)]
pub struct ConnectionFailure {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    duration: Duration,
}

impl ConnectionFailure {

    /// Constructs a new ConnectionFailure.
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        let duration = end - start;

        ConnectionFailure {
            start,
            end,
            duration,
        }
    }
}
