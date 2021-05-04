use std::slice;
use std::ops::Add;
use std::fmt::{Display, Formatter, Result};

#[allow(dead_code)]
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr, len - mid),
        )
    }
}

trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> Self {
        *self * 2
    }
}

impl Double for f64 {
    fn double(&self) -> Self {
        *self * 2_f64
    }
}

impl Double for String {
    fn double(&self) -> Self {
        let mut s = self.clone();
        s.push_str(&self);
        s
    }
}

#[derive(Copy)]
struct Millimeters(u32);

#[derive(Copy)]
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Self) -> Self::Output {
        Millimeters(self.0 + rhs.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + 1000 * rhs.0)
    }
}

impl Display for Millimeters {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} mm", self.0)
    }
}

impl Display for Meters {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} m", self.0)
    }
}

impl Clone for Millimeters {
    fn clone(&self) -> Self {
        Millimeters(self.0)
    }
}

impl Clone for Meters {
    fn clone(&self) -> Self {
        Meters(self.0)
    }
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat((len + 2)));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn higher_order() -> fn(&str) {
    |s| println!("{}", s)
}

fn main() {

    //-------------- RAW POINTERS-----------
    // let x = 5;
    //
    // let x_ptr = &x as *const i32;
    //
    // println!("x: {}, x_ptr: {:?}", x, x_ptr);

    // ------------- CUSTOM TRAIT IMPLEMENTATIONS ---------------------
    // println!("{} doubled is {}", 2, 2.double());
    // println!("{} doubled is {}", 3.5, 3.5.double());
    //
    // let bla = String::from("Bla");
    //
    // println!("{} doubled is {}", bla, bla.double());

    // -------------- OPERATOR OVERLOAD --------------------
    // let a = Millimeters(10);
    // let b = Millimeters(5);
    // let c = Meters(1);
    //
    // println!("{} + {} is {}", a, b, a + b);
    // println!("{} + {} is {}", a, c, a + c);

    // --------------- SUPER TRAITS -----------------
    // let point = Point {
    //     x: 1,
    //     y: 3
    // };
    //
    // point.outline_print();

    // -------------- HIGHER ORDER FUNCTIONS ---------------
    let some_func = higher_order();

    some_func("hello there");
}
