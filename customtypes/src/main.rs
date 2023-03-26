#![allow(dead_code)]
fn main() {
    // struct
    // 1. tuple struct
    struct Pair(i32, f32);
    // 2. c struct
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    // 3. unit struct
    struct Unit;

    // enum
    // create a type may be one of a few different variants.
    #[derive(Debug)]
    enum WebEvent {
        // unit struct like
        PageLoad,
        PageUnload,
        // tuple struct like
        KeyPress(char),
        Paste(String),
        // c struct like
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) -> () {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'", c),
            WebEvent::Paste(s) => println!("pasted '{}'", s),
            WebEvent::Click { x, y } => {
                println!("cliked at x={},y={}", x, y);
            }
        }
    }

    // type aliase
    type WEvent = WebEvent;

    // linked-list
    enum List {
        Nil,
        Cons(u32, Box<List>),
    }
    impl List {
        fn new() -> List {
            List::Nil
        }

        fn prepend(self, elem: u32) -> List {
            Self::Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }
        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    format!("{} {}", head, tail.stringify())
                }
                List::Nil => format!("Nil"),
            }
        }
    }

    // const, static
    // const: an unchangeable value
    // static: a possibly muable variable with 'static lifetime, which is inferred.
}
