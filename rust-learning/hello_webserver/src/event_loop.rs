//! Event loop inspired by JavaScript promises

use std::marker::PhantomData;

struct Promise<T> {
    result: T,
}

enum PromiseResult<T> {
    Pure(T),
    Promise(Promise<T>),
}

impl<T> PromiseResult<T> {
    pub fn new<R: ToPromiseResult<T>>(result: R) -> Self {
        result.into_promise_result()
    }
}

trait ToPromiseResult<T> {
    fn into_promise_result(self) -> PromiseResult<T>;
}

impl<T> ToPromiseResult<T> for T where T: Copy {
    fn into_promise_result(self) -> PromiseResult<T> {
        PromiseResult::Pure(self)
    }
}

impl<T> ToPromiseResult<T> for Promise<T> {
    fn into_promise_result(self) -> PromiseResult<T> {
        PromiseResult::Promise(self)
    }
}

fn test() {
    let r1 = PromiseResult::new(5);
    let r2 = PromiseResult::new(Promise { result: 5 });
}
