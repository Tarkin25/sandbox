use crate::ProbeConnection;
use std::time::Duration;
use std::thread;

pub struct ConnectionChecker<'a> {
    failure_listeners: Vec<Box<dyn FnMut() + 'a>>,
    reconnect_listeners: Vec<Box<dyn FnMut() + 'a>>,
    probe: Box<dyn ProbeConnection + 'a>,
    probe_interval: Duration,
    running: bool,
}

impl<'a> ConnectionChecker<'a> {
    pub fn builder() -> Builder<'a> {
        Builder::new()
    }

    pub async fn start(&mut self) {
        self.running = true;

        println!("started");

        while self.running {
            self.until_failure().await;

            self.on_connection_failure();

            self.reconnect().await;

            self.on_reconnect();
        }

        println!("stopped");
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    fn on_connection_failure(&mut self) {
        for listener in &mut self.failure_listeners {
            listener();
        }
    }

    fn on_reconnect(&mut self) {
        for listener in &mut self.reconnect_listeners {
            listener();
        }
    }

    async fn until_failure(&self) {
        loop {
            if self.probe.connection_success().await {
                println!("Connection ok.");

                thread::sleep(self.probe_interval);
            } else {
                break;
            }
        }
    }

    async fn reconnect(&self) {
        loop {
            if !self.probe.connection_success().await {
                println!("Re-connect failed.");

                thread::sleep(self.probe_interval);
            } else {
                break;
            }
        }
    }
}

pub struct Builder<'a> {
    failure_listeners: Vec<Box<dyn FnMut() + 'a>>,
    reconnect_listeners: Vec<Box<dyn FnMut() + 'a>>,
    probe: Option<Box<dyn ProbeConnection + 'a>>,
    probe_interval: Option<Duration>,
}

impl<'a> Builder<'a> {
    fn new() -> Builder<'a> {
        Builder {
            failure_listeners: vec![],
            reconnect_listeners: vec![],
            probe: None,
            probe_interval: None,
        }
    }

    pub fn on_connection_failure<L>(mut self, listener: L) -> Builder<'a>
    where L: FnMut() + 'a
    {
        self.failure_listeners.push(Box::new(listener));

        self
    }

    pub fn on_reconnect<L>(mut self, listener: L) -> Builder<'a>
    where L: FnMut() + 'a
    {
        self.reconnect_listeners.push(Box::new(listener));

        self
    }

    pub fn probe<P>(mut self, probe: P) -> Builder<'a>
    where P: ProbeConnection + 'a
    {
        self.probe = Some(Box::new(probe));

        self
    }

    pub fn probe_interval(mut self, interval: Duration) -> Builder<'a> {
        self.probe_interval = Some(interval);

        self
    }

    /// Builds a ConnectionChecker
    ///
    /// # Panics
    /// - if no probe is configured
    /// - if no probe_interval is configured
    pub fn build(self) -> ConnectionChecker<'a> {
        let Builder { failure_listeners, reconnect_listeners, probe, probe_interval } = self;

        assert!(probe.is_some(), "probe is required");
        assert!(probe_interval.is_some(), "probe_interval is required");

        let probe = probe.unwrap();
        let probe_interval = probe_interval.unwrap();

        ConnectionChecker {
            failure_listeners,
            reconnect_listeners,
            probe,
            probe_interval,
            running: false
        }
    }
}

#[cfg(test)]
mod checker_tests {
    use crate::{ProbeConnection, ConnectionChecker};
    use async_trait::async_trait;
    use std::cell::RefCell;
    use std::time::Duration;
    use std::thread;
    use std::sync::Mutex;
    use std::borrow::{Borrow, BorrowMut};
    use std::ops::{DerefMut, Deref};

    enum MockProbeStatus {
        ConnectSuccess,
        ConnectFailure,
        ReconnectFailure,
        ReconnectSuccess,
    }

    struct MockProbe {
        status: Mutex<MockProbeStatus>,
    }

    impl MockProbe {
        fn new() -> MockProbe {
            MockProbe {
                status: Mutex::new(MockProbeStatus::ConnectSuccess)
            }
        }
    }

    #[async_trait]
    impl ProbeConnection for MockProbe {
        async fn connection_success(&self) -> bool {
            use MockProbeStatus::*;

            let success = match *self.status.lock().unwrap().deref() {
                ConnectSuccess | ReconnectSuccess => true,
                ConnectFailure | ReconnectFailure => false,
            };

            let new_status = match *self.status.lock().unwrap().deref() {
                ConnectSuccess => ConnectFailure,
                ConnectFailure => ReconnectFailure,
                ReconnectFailure => ReconnectSuccess,
                ReconnectSuccess => ReconnectSuccess,
            };

            *self.status.lock().unwrap().deref_mut() = new_status;

            success
        }
    }

    #[tokio::test]
    async fn calls_failure_listeners_on_connection_failure() {
        let failure_listener_called = Mutex::new(false);

        let mut checker = ConnectionChecker::builder()
            .probe(MockProbe::new())
            .probe_interval(Duration::from_millis(1))
            .on_connection_failure(|| {
                *failure_listener_called.lock().unwrap().deref_mut() = true;
            })
            .build();

        checker.start().await;

        thread::sleep(Duration::from_millis(5000));
        //
        // checker.stop();
        //
        // let called = *failure_listener_called.lock().unwrap().deref();
        //
        // assert!(dbg!(called));
    }

}