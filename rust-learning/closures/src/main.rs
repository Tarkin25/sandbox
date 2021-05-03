use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, A, R>
where
    T: Fn(A) -> R,
    A: Eq + Hash,
    R: Copy
{
    calculation: T,
    results: HashMap<A, R>,
}

impl <T, A, R> Cacher<T, A, R>
where
    T: Fn(A) -> R,
    A: Eq + Hash + Copy,
    R: Copy
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            results: HashMap::new()
        }
    }

    fn result(&mut self, arg: A) -> R {
        *self.results.entry(arg).or_insert((self.calculation)(arg))
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.result(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.result(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.result(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use crate::Cacher;

    #[test]
    fn call_with_different_values() {
        let mut cacher = Cacher::new(|a| a);

        let _v1 = cacher.result(1);
        let v2 = cacher.result(2);

        assert_eq!(2, v2);
    }
}
