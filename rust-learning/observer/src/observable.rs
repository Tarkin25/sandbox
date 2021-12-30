use std::collections::HashMap;
use crate::{OnEvent, Subscription};

pub struct Observable<T> {
    subscriptions: HashMap<Subscription, Box<dyn OnEvent<T>>>,
}

impl<T> Observable<T> {
    pub fn subscribe(&mut self, on_event: impl OnEvent<T> + 'static) -> Subscription {
        let sub = Subscription::new();
        self.subscriptions.insert(sub, Box::new(on_event));
        sub
    }

    pub fn emit_event(&mut self, event: T) where T: Copy {
        self.subscriptions
            .values_mut()
            .for_each(|on_event| on_event.on_event(event));
    }

    pub fn unsubscribe(&mut self, subscription: Subscription) {
        self.subscriptions
            .remove(&subscription);
    }
}

impl<T> Default for Observable<T> {
    fn default() -> Self {
        Self {
            subscriptions: Default::default(),
        }
    }
}
