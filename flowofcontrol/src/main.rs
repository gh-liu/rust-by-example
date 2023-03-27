fn main() {
    let n = 5;
    if 0 > n {
        println!("{} is negative", n);
    } else {
        println!("{} is positive", n);
    }

    // ifelse conditions are expressions
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    println!("{} -> {}", n, big_n);

    // loop: an infinite loop
    // break: exit a loop;
    // continue: skip the rest of the iteration and start a new one.
    let mut count = 0u32;
    loop {
        count += 1;
        if count == 3 {
            continue;
        }
        if count == 5 {
            break;
        }
    }
    println!("{}", count);

    // nesting and labels
    'outer: loop {
        println!("outer loop");

        'inner: loop {
            println!("inner loop");
            break 'outer;
        }
        println!("never be reached");
    }

    // returning from loops
    // put value after break
    let mut counter = 20;
    let result = loop {
        counter += 1;
        if counter == 30 {
            break counter * 3;
        }
    };
    println!("{}", result);

    // while
    // while true {
    //     unimplemented!();
    // }

    // for and range
    // `for in` construct can be used to iterate through an `Iterator`
    for n in 1..10 {
        println!("{}", n);
    }
    // = will include the end
    for n in 1..=10 {
        println!("{}", n);
    }
    // `for` will apply the `into_iter` function to the collection.
    // into_iter, iter, iter_mut will provide different views on the data.
    // iter: borrow each element of the colletcion;
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // into_iter: consumes(move) the collection;
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names);

    // iter_mut: mutably borrow each element of the collection.
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    match_demo();

    // if let
    let number = Some(7);
    if let Some(pat) = number {
        println!("{}", pat)
    }
    enum Foo {
        Bar,
    }
    let a = Foo::Bar;
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    // let else

    // while let
}

fn match_demo() {
    // match
    // pattern matching
    // which is an expression
    let boolean = true;
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    // 1. destructuring
    // truples, arrays/slices, enums, pointers, structures
    let triple = (0, -2, 3);
    match triple {
        (0, y, z) => println!("y is {} and z is {}", y, z),
        (1, ..) => println!("1st is 1, and the rest does not matter"),
        (.., 2) => println!("last is 2, and the rest does not matter"),
        (3, .., 4) => println!("1st is 3 and last is 4, and the rest does not matter"),
        _ => println!("it does not matter what they are"),
    }
    let array = [1, -2, 6];
    match array {
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        // store them in another array/slice
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
    }
    // NOTE:
    // Dereferencing uses *
    // Destructuring uses &, ref, and ref mut
    let reference = &4;
    match reference {
        &val => println!("destructuring: {}", val),
    }
    match *reference {
        reference => println!("dererferencing: {}", reference),
    }
    let _not_a_reference = 3;
    // `ref` modifies the assignment
    // so that a reference is create for the element.
    let ref _a_refenece = 3;
    let value = 5;
    match value {
        ref r => println!("get a reference to a value {:?}", r),
    }
    let mut mut_val = 6;
    match mut_val {
        ref mut m => {
            *m += 10;
            println!("add 10 for mut_val: {:?}", m)
        }
    }
    #[derive(Debug)]
    struct Foo {
        x: (u32, u32),
    }
    let foo = Foo { x: (1, 2) };
    match foo {
        Foo { x: (1, b) } => println!("{}", b),
        _ => println!("does not matter"),
    }

    // 2. gurads
    // to filter the arm
    #[derive(Debug)]
    enum Temperature {
        Celsius(i32),
        Fahrenhit(i32),
    }
    let temp = Temperature::Celsius(35);
    match temp {
        Temperature::Celsius(t) if t > 30 => println!("{} > 30", t),
        _ => println!("does not matter"),
    }

    // 3. binding
    fn age() -> u32 {
        15
    }
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
    fn some_number() -> Option<u32> {
        Some(42)
    }
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}
