use std::mem;

fn main() {
    // Scalar Types
    // signed intergers
    // unsigned intergers
    // floating point
    // char
    // bool
    // unit type

    // Compound Types
    // arrays
    // tuples

    let _logical: bool = true;
    let _a_float: f64 = 1.0;
    let _an_interger = 5i32;

    let _default_float = 3.0;
    let _default_integer = 7;

    let mut _inferred_type = 12;
    _inferred_type = 4294967296i64;

    let mut _mutable = 12;
    _mutable = 21;

    // type can not be changed.
    // _mutable = true;

    // can be overwitten with shadowing.
    let _mutable = true;

    // literals, operators
    println!("1+2 = {}", 1u32 + 2);
    println!("true AND false is {}", true && false);
    println!("one million is written as {}", 1_000_000u32);

    // tuples: A tuple is a collection of values of different types.
    // as function parameters or as return valuse
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (int_p, bool_p) = pair;
        (bool_p, int_p)
    }

    // arrays, slices
    // the length of arrays is known at compile time, slices it not;
    // slice is a two word object, the 1st word is a pointer to the data,
    // the 2nd word is the length of the slice.
    // slice can be used to borrow a section of a array
    // and have the type signature &[T].
    fn analyze_slice(slice: &[i32]) -> () {
        println!("1st element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }
    let xs = [1, 2, 3, 4, 5];
    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("slow down! {} is too far", i),
        }
    }
}
