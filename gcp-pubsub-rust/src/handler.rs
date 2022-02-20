use crate::{FromMessage};
use futures::future::BoxFuture;
use google_cloud::pubsub::Message;
use std::future::Future;
use std::marker::PhantomData;

pub trait RawHandler: Send {
    fn on_message(&self, message: Message) -> BoxFuture<'static, ()>;
}

pub trait HandlerFuture: Future<Output=()> + Send + 'static {}

impl<F> HandlerFuture for F where F: Future<Output=()> + Send + 'static {}

pub trait Handler<T, Fut>: Send + 'static
where
    Fut: HandlerFuture
{
    fn call(&self, param: T) -> Fut;
}

impl<Fut, F> Handler<(), Fut> for F
where
    Fut: HandlerFuture,
    F: Fn() -> Fut + Send + 'static,
{
    fn call(&self, _param: ()) -> Fut {
        self()
    }
}

impl<Fut, F, T> Handler<(T,), Fut> for F
where
    Fut: HandlerFuture,
    F: Fn(T) -> Fut + Send + 'static,
{
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
    H: Handler<T, Fut>
{
    fn on_message(&self, message: Message) -> BoxFuture<'static, ()> {
        let message = T::from_message(message).expect("Failed to convert message");

        Box::pin(self.handler.call(message))
    }
}