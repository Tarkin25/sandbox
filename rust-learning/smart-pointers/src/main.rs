use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::mem;
use std::rc::{Rc, Weak};

struct MyBox<T>(T)
where
    T: Display;

impl<T> MyBox<T>
where
    T: Display,
{
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where
    T: Display,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T>
where
    T: Display,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Drop for MyBox<T>
where
    T: Display,
{
    fn drop(&mut self) {
        println!("Dropping MyBox with data '{}'!", self.0);
    }
}

fn my_box_demo() {
    let x = MyBox::new("stuff");
    let mut y = MyBox::new(*x);

    println!("x: {}, y: {}", *x, *y);

    *y = "other stuff";

    println!("x: {}, y: {}", *x, *y);

    mem::drop(x);
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::cell::RefCell;

fn list_demo() {
    use crate::List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("count after creating a = {}", Rc::strong_count(&a));

    // We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case.
    // The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.
    // The call to Rc::clone only increments the reference count, which doesn’t take much time.
    // Deep copies of data can take a lot of time.
    // By using Rc::clone for reference counting, we can visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count.
    // When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to Rc::clone.
    let b = Cons(3, Rc::clone(&a));

    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));

        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes ouf of scope = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum ModifiableList {
    Cons(Rc<RefCell<i32>>, Rc<ModifiableList>),
    Nil,
}

fn modifiable_list_demo() {
    use crate::ModifiableList::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn tree_demo() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn main() {
    tree_demo();
}
