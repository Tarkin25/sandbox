use crate::{FromMessage, ProcessingSignal};
use futures::future::BoxFuture;
use google_cloud::pubsub::Message;
use std::future::Future;
use std::marker::PhantomData;
use anyhow::Context;

pub trait RawHandler: Send {
    fn on_message(&self, message: Message) -> BoxFuture<'static, anyhow::Result<ProcessingSignal>>;
}

pub trait HandlerFuture: Future<Output=anyhow::Result<ProcessingSignal>> + Send + 'static {}

impl<F> HandlerFuture for F where F: Future<Output=anyhow::Result<ProcessingSignal>> + Send + 'static {}

pub trait Handler<T>: Send + 'static
{
    type Future: HandlerFuture;

    fn call(&self, param: T) -> Self::Future;
}

impl<Fut, F, T> Handler<(T,)> for F
where
    Fut: HandlerFuture,
    F: Fn(T) -> Fut + Send + 'static,
{
    type Future = Fut;

    fn call(&self, param: (T,)) -> Fut {
        self(param.0)
    }
}

pub struct Extract<T, Fut, H> {
    handler: H,
    phantom_data: PhantomData<(T, Fut)>
}

impl<T, Fut, H> Extract<T, Fut, H> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            phantom_data: PhantomData,
        }
    }
}

impl<T, Fut, H> RawHandler for Extract<T, Fut, H>
where
    T: FromMessage + Send,
    Fut: HandlerFuture,
    H: Handler<T, Future=Fut>
{
    fn on_message(&self, message: Message) -> BoxFuture<'static, anyhow::Result<ProcessingSignal>> {
        let message_result = T::from_message(message).context("Failed to extract message");

        match message_result {
            Ok(message) => Box::pin(self.handler.call(message)),
            Err(extraction_error) => Box::pin(async { Err(extraction_error) })
        }
    }
}