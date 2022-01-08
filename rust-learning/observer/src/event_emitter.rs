use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use crate::{OnEvent, Observable, Subscription};

pub struct EventEmitter<'a, K, E>
where
    K: Eq + Hash,
{
    observables: HashMap<K, Observable<'a, E>>,
}

impl<'a, K, E> EventEmitter<'a, K, E> where K: Eq + Hash {

    pub fn new() -> Self {
        Default::default()
    }

    pub fn subscribe(&mut self, event_type: K, on_event: &'a mut dyn OnEvent<E>) -> Subscription {
        self.observables
            .entry(event_type)
            .or_default()
            .subscribe(on_event)
    }

    pub fn unsubscribe(&mut self, event_type: impl Borrow<K>, subscription: Subscription) {
        if let Some(observable) = self.observables.get_mut(event_type.borrow()) {
            observable.unsubscribe(subscription);
        }
    }

    pub fn emit_event(&mut self, event_type: impl Borrow<K>, event: E) where E: Clone {
        if let Some(observable) = self.observables.get_mut(event_type.borrow()) {
            observable.emit_event(event);
        }
    }

}

impl<K, E> Default for EventEmitter<'_, K, E> where K: Eq + Hash {
    fn default() -> Self {
        Self {
            observables: HashMap::default(),
        }
    }
}