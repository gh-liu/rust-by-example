fn main() {
    // conversion between custom types
    // use `From` and `Into` traits.

    // From: allows for a type to define how to create itself from another type.
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number { value: value }
        }
    }
    let num = Number::from(30);
    println!("my number is {:?}", num);

    // Info is simply the reciprocal of the From.
    let i = 5;
    let n: Number = i.into();
    println!("my number is {:?}", n);

    // `TryFrom` and `TryInto` are generic traits for converting between types.
    // used for fallible conversions.
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);
    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }
    assert_eq!(EvenNumber::try_from(8),Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5),Err(()));
    let res:Result<EvenNumber,()> = 8i32.try_into();
    assert_eq!(res, Ok(EvenNumber(8)));
    let res:Result<EvenNumber,()> = 7i32.try_into();
    assert_eq!(res, Err(()));

    // To and From strings
    // To: impl the `ToString` trait,
    // impl the fmt::Display trait which automagically provides Tostring.

    // From: use parse function or impl the `FromStr` trait.

}
