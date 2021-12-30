use uuid::Uuid;
use crate::Observable;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Subscription(Uuid);

impl Subscription {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn unsubscribe<T>(self, observable: &mut Observable<T>) {
        observable.unsubscribe(self);
    }
}