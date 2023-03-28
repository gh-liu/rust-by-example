// raii.rs
fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }
    // `_box2` is destroyed here, and memory gets freed
}

// scopes playa an important part in ownership, borrowing, and lifetimes.

// RAII: Resource Acquisition Is Initializtion.
// Whenever an object goes out of scope,
// its destructor is called and its owned resources are freed.
// The notion of a destructor in Rust is provided through the Drop trait.
// struct ToDrop;
// impl Drop for ToDrop {
//     fn drop(&mut self) {
//         println!("ToDrop is being dropped");
//     }
// }
// fn main() {
//     let x = ToDrop;
//     println!("Made a ToDrop!");
// }

// Ownership:
// Variables are in charge of freeing their own resources, resources can only have one owner.
// Note that not all variables own resources (e.g. references).
// move: the ownership of the resources is transferred.
// After moving resources, the previous owner can no longer be used.
// partial move: Within the destructuring of a single variable,
// both by-move and by-reference pattern bindings can be used at the same time.

// Borrowing: &T
// The compiler statically guarantees (via its borrow checker) that references always point to valid objects.
// mmutably borrowed any number of times, or only one mutable borrow at a time.
// ref: pattern matching or destructuing via the let binding,
// ref can be used to take references to the fields.

// Lifetimes:
// 1. explicit annotation: determine how long references should be valid.
// foo<'a>: `foo` has a lifetime parameter `'a`
// foo<'a, 'b>: the lifetime of foo cannot exceed that of either 'a or 'b.
// the form &'a T where 'a has already been introduced.
// 2. function
// any reference must have an annotated lifetime.
// any reference being returned must have the same lifetime as an input or be static.
// 3. method
// similarly to functions
// 4. struct
// similarly to functions
// 5. trait
// basically similar to functions. Note that impl may have annotation of lifetimes too.
// 6. bounds
// T: 'a: All references in T must outlive lifetime 'a.
// T: Trait + 'a: Type T must implement trait Trait and all references in T must outlive 'a.
// 7. coercoin
// A longer lifetime can be coerced into a shorter one.
// 8. static
fn _staticd() -> () {
    // A reference with 'static lifetime:
    let s: &'static str = "hello world";
    // 'static as part of a trait bound:
    fn generic<T>(x: T)
    where
        T: 'static,
    {
    }
}
// 9. elision
