use reqwest::{Client, Request};
use std::time::Duration;
use async_trait::async_trait;

#[async_trait]
pub trait ProbeConnection {
    async fn connection_success(&self) -> bool;
}

pub struct HttpSingleProbe<'a> {
    client: Client,
    url: &'a str,
}

impl<'a> HttpSingleProbe<'a> {
    pub fn new(url: &'a str, timeout: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap();

        HttpSingleProbe {
            client,
            url,
        }
    }

    fn create_request(&self) -> Request {
        self.client
            .get(self.url)
            .build()
            .unwrap()
    }
}

#[async_trait]
impl<'a> ProbeConnection for HttpSingleProbe<'a> {
    async fn connection_success(&self) -> bool {
        let request = self.create_request();

        match self.client.execute(request).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
