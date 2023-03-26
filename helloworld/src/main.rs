use core::fmt;

fn main() {
    println!("Hello, world!");

    println!("{} days", 31);
    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // named arguments
    println!(
        "{subject} {verb} {object}",
        subject = "the lazy dog",
        object = "the quick brown fox",
        verb = "jumps over",
    );
    // formater character
    println!("base 10: {}", 69420);
    println!("base 2 (binary): {:b}", 69420);
    println!("base 8 (octal): {:o}", 69420);
    println!("base 16 (hex): {:x}", 69420);
    println!("base 16 (hex): {:X}", 69420);

    println!("{number:>5}", number = 1); // "     1"
    println!("{number:0<5}", number = 1); // "100000"
    println!("{number:0<width$}", number = 1, width = 6); // "1000000"

    // printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year", 12);
    // The `derive` attribute automatically creates the implemention
    // required to make this `struct` printable with `fmt.Debug`.
    #[derive(Debug)]
    struct Structure(i32);
    #[derive(Debug)]
    struct Deep(Structure);
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(3)));
    #[derive(Debug)]
    struct Persion<'a> {
        name: &'a str,
        age: u8,
    }
    let perter = Persion {
        name: "petter",
        age: 27,
    };
    // pretty printing
    println!("{:#?}", perter);

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }
    let minmax = MinMax(0, 14);
    println!("Debug: {:?}", minmax);
    println!("Display: {}", minmax);

    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }
            write!(f, "]")
        }
    }
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
