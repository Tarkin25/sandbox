use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use crate::{OnEvent, Observable, Subscription};

pub struct EventEmitter<K, E>
where
    K: Eq + Hash,
{
    observables: HashMap<K, Observable<E>>,
}

impl<K, E> EventEmitter<K, E> where K: Eq + Hash {

    pub fn new() -> Self {
        Default::default()
    }

    pub fn subscribe(&mut self, event_type: K, observe: impl OnEvent<E> + 'static) -> Subscription {
        self.observables
            .entry(event_type)
            .or_default()
            .subscribe(observe)
    }

    pub fn unsubscribe(&mut self, event_type: impl Borrow<K>, subscription: Subscription) {
        if let Some(observable) = self.observables.get_mut(event_type.borrow()) {
            observable.unsubscribe(subscription);
        }
    }

    pub fn emit_event(&mut self, event_type: impl Borrow<K>, event: E) where E: Copy {
        if let Some(observable) = self.observables.get_mut(event_type.borrow()) {
            observable.emit_event(event);
        }
    }

}

impl<K, E> Default for EventEmitter<K, E> where K: Eq + Hash {
    fn default() -> Self {
        Self {
            observables: HashMap::default(),
        }
    }
}