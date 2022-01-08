fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

macro_rules! compose {
    ($last: expr) => { $last };
    ($head: expr, $($tail: expr), +) => {
        compose_two($head, compose!($($tail), +))
    };
}

fn curry<F, A, B, C>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    A: Copy + 'static,
    F: Fn(A, B) -> C + Copy + 'static,
{
    move |x| Box::new(move |y| f(x, y))
}

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        Self { id: 1, name }
    }
}

fn store_user(user: User) -> User {
    println!("Stored {:?}", user);

    user
}

trait Compose<A, B>: Fn(A) -> B {
    fn and_then<C, F>(self, f: F) -> Box<dyn Fn(A) -> C>
    where
        F: Fn(B) -> C + 'static,
        Self: Sized + 'static,
    {
        Box::new(move |a| f(self(a)))
    }
}

impl<A, B, F> Compose<A, B> for F where F: Fn(A) -> B {}

trait Curry<A, B, C>: Fn(A, B) -> C {
    fn with(self, a: A) -> Box<dyn Fn(B) -> C>
    where
        A: Copy + 'static,
        Self: Copy + 'static,
    {
        Box::new(move |b| self(a, b))
    }
}

impl<A, B, C, F> Curry<A, B, C> for F where F: Fn(A, B) -> C {}

fn main() {
    // composition
    let create_user = compose!(User::new, store_user);
    create_user("John Doe".into());

    let create_user = User::new.and_then(store_user);
    create_user("Jane Smith".into());

    // currying
    let add = |a, b| a + b;
    let add_five = add.with(5);
    println!("Result: {}", add_five(5));

    // combined
    let trace = |tag: &'static str, x| {
        println!("{} {:?}", tag, x);
        x
    };
    let finally = add
        .with(5)
        .and_then(trace.with("after add:"))
        .and_then(|x| x * 2)
        .and_then(trace.with("after double:"))
    ;
    finally(5);
}
