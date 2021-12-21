use futures::{FutureExt, StreamExt};
use futures::prelude::*;

async fn get_all() -> Vec<i32> {
    vec![1,2,3,4]
}

async fn double(number: i32) -> i32 {
    number * 2
}

#[tokio::main]
async fn main() {
    let nums = get_all().await;
    let nums = stream::iter(nums);

    let doubled = nums.then(double);

    let result = doubled.collect::<Vec<_>>().await;

    println!("result: {:?}", result);
}
