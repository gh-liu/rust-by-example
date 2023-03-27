use std::ops::Sub;

fn main() {
    // the final expression in the function will be used as return value
    ()
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
// associated functions, methods
impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn down(&mut self) {
        self.y -= 1f64
    }
}

// 1. closures
fn caputuring() {
    // closures
    // |val| val+x
    let out_val = 5;
    let closure_inferred = |i: i32| i + out_val;
    closure_inferred(6);

    // capturing
    // ref: `&T`, mut ref: `&mut T`, valu: `T`
    // 优先使用ref，只在需要时，往后选择
    let color = String::from("green");
    let print = || println!("`color`: {}", color);
    print();
    let _reborrow = &color;
    print();
    let _color_moved = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    // Call the closure using a mutable borrow
    inc();
    // let _reborrow = &count;
    inc();
    // The closure no longer needs to borrow `&mut count`.
    let _count_reborrowed = &mut count;
    use std::mem;
    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume();
}
// as input parameters, the closure's complete type must be annotated;
// use one of a few traits: Fn, FnMut, FnOnce
// Fn: &T
// FnMut: &mut T
// FnOnce: T
// the compiler will capture variables in the least restrictive manner possible.
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f()
}
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}
// type anonymity
// When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variables inside, meanwhile implementing the functionality via one of the traits: Fn, FnMut, or FnOnce for this unknown type.
// This type is assigned to the variable which is stored until calling.

// input functions
// any function that satisfies the trait bound of that closure can be passed as a parameter.

// as output parameters,
// anonymous closure types are unknown, so we have to use impl Trait to return them.
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

// 2. high order functions

// 3. diverging functions
