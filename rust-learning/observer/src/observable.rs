use std::collections::HashMap;
use crate::{OnEvent, Subscription};

pub struct Observable<'a, T> {
    subscriptions: HashMap<Subscription, &'a mut dyn OnEvent<T>>,
}

impl<'a, T> Observable<'a, T> {
    pub fn subscribe(&mut self, on_event: &'a mut dyn OnEvent<T>) -> Subscription {
        let sub = Subscription::new();
        self.subscriptions.insert(sub, on_event);
        sub
    }

    pub fn emit_event(&mut self, event: T) where T: Clone {
        self.subscriptions
            .values_mut()
            .for_each(|on_event| on_event.on_event(event.clone()));
    }

    pub fn unsubscribe(&mut self, subscription: Subscription) {
        self.subscriptions
            .remove(&subscription);
    }
}

impl<T> Default for Observable<'_, T> {
    fn default() -> Self {
        Self {
            subscriptions: Default::default(),
        }
    }
}
