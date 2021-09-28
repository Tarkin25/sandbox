use std::fmt::Debug;
use chrono::{DateTime, Utc, Duration};

mod log;
mod probe;
mod check;
mod track;

pub use log::*;
pub use probe::*;
pub use check::*;
pub use track::*;

#[derive(Debug, PartialEq)]
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
