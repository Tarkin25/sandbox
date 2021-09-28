use std::cell::RefCell;
use chrono::{DateTime, Utc};
use crate::ConnectionFailure;

pub struct FailureTracker {
    failure_start: RefCell<Option<DateTime<Utc>>>,
    failure_end: RefCell<Option<DateTime<Utc>>>,
}

impl FailureTracker {
    pub fn new() -> Self {
        FailureTracker {
            failure_start: RefCell::new(None),
            failure_end: RefCell::new(None)
        }
    }

    pub fn failure_start(&self, timestamp: DateTime<Utc>) {
        *self.failure_start.borrow_mut() = Some(timestamp);
    }

    pub fn failure_end(&self, timestamp: DateTime<Utc>) {
        *self.failure_end.borrow_mut() = Some(timestamp);
    }

    pub fn current_failure(&self) -> Option<ConnectionFailure> {
        if let Some(start) = *self.failure_start.borrow() {
            if let Some(end) = *self.failure_end.borrow() {
                return Some(ConnectionFailure::new(start, end));
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::FailureTracker;
    use chrono::{Utc};

    #[test]
    fn failure_start_then_current_failure_is_none() {
        let tracker = FailureTracker::new();

        tracker.failure_start(Utc::now());

        assert_eq!(None, tracker.current_failure());
    }

    #[test]
    fn failure_end_then_current_failure_is_none() {
        let tracker = FailureTracker::new();

        tracker.failure_end(Utc::now());

        assert_eq!(None, tracker.current_failure());
    }

    #[test]
    fn failure_start_then_failure_end_then_current_failure_is_correct() {
        let tracker = FailureTracker::new();

        let start = Utc::now();
        let end = Utc::now();

        tracker.failure_start(start);
        tracker.failure_end(end);

        let failure = tracker.current_failure();

        assert!(failure.is_some());

        let failure = failure.unwrap();

        assert_eq!(start, failure.start);
        assert_eq!(end, failure.end);
    }

}
