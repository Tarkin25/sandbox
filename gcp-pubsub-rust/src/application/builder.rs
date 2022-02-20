use std::future::Future;
use crate::application::Listener;
use crate::{Application, FromMessage};
use crate::handler::{Extract, Handler, RawHandler};

#[derive(Default)]
pub struct ApplicationBuilder {
    pub(crate) listeners: Vec<Listener>,
}

impl ApplicationBuilder {
    pub(crate) fn new() -> Self {
        Self { listeners: vec![] }
    }

    pub async fn start(self) -> Result<Application, Box<dyn std::error::Error>> {
        Application::start(self).await
    }

    fn add_listener<T: RawHandler + 'static>(mut self, topic: impl Into<String>, subscription: impl Into<String>, handler: T) -> Self {
        self.listeners.push(Listener {
            topic: topic.into(),
            subscription: subscription.into(),
            handler: Box::new(handler)
        });

        self
    }

    pub fn listen<T, Fut, H>(self, topic: impl Into<String>, subscription: impl Into<String>, handler: H) -> Self
    where
        T: FromMessage + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
        H: Handler<T, Fut>,
    {
        self.add_listener(topic, subscription, Extract::new(handler))
    }
}
