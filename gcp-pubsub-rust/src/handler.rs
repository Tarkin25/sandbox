use crate::{FromMessage, ProcessingResult};
use futures::future::BoxFuture;
use google_cloud::pubsub::Message;
use std::future::Future;
use std::marker::PhantomData;

pub trait RawHandler: Send {
    fn on_message(&self, message: Message) -> BoxFuture<'static, ProcessingResult>;
}

pub trait HandlerFuture: Future<Output=ProcessingResult> + Send + 'static {}

impl<F> HandlerFuture for F where F: Future<Output=ProcessingResult> + Send + 'static {}

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
    fn on_message(&self, message: Message) -> BoxFuture<'static, ProcessingResult> {
        let message = T::from_message(message).expect("Failed to convert message");

        Box::pin(self.handler.call(message))
    }
}