fn main() {
    // type parameters

    fn foo<T>(arg: T) -> () {
        unimplemented!();
    }

    struct SingleGen<T>(T);
    let _s = SingleGen('a');

    // Generic type `SGen`.
    struct SGen<T>(T);
    // 1.funtions
    fn generic<T>(_s: SGen<T>) {}
    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));
    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));

    // 2. implemention
    // Generic type `GenericVal`
    struct GenericVal<T>(T);
    // impl of GenericVal where we explicitly specify type parameters:
    // Specify `f32`
    impl GenericVal<f32> {}
    // `<T>` Must precede the type to remain generic
    impl<T> GenericVal<T> {}

    // 3. traits
    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }
    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {}
    }

    // 3. bounds
    // the type parameters often must use traits as bounds
    // to stipulate what functionality a type implements.
    use std::fmt::Display;
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }

    // 4. multiple bounds
    // Multiple bounds for a single type can be applied with a +.

    // 5. Where clauses
    // A bound can also be expressed using a where clause immediately before the opening { ;
    // where clauses can apply bounds to arbitrary types, rather than just to type parameters.
    use std::fmt::Debug;
    trait PrintInOption {
        fn print_in_option(self);
    }
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        // We want `Option<T>: Debug` as our bound because that is what's
        // being printed. Doing otherwise would be using the wrong bound.
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    // 6. New type idiom

    // 7. Associated Items
    // an extension to trait generics

    // 8. Phantom type parameters
    // A phantom type parameter is one that doesn't show up at runtime,
    // but is checked statically (and at compile time.
}
