fn main() {
    // casting: use as keyword
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("casing: {} -> {} -> {}", decimal, integer, character);

    // literals
    // adding type as a suffix
    let x = 1u8;
    let y = 2u8;
    let z = 3f32;
    println!("size of `x` {}", std::mem::size_of_val(&x));
    println!("size of `y` {}", std::mem::size_of_val(&y));
    println!("size of `z` {}", std::mem::size_of_val(&z));

    // type inference
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    // aliasing types
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanosecond: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanosecond,
        inches,
        nanosecond + inches
    );
}
