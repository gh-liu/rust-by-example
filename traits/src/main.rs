fn main() {
    // A trait is a collection of methods defined for an unknown type: Self.
    // They can access other methods declared in the same trait.
    // Traits can be implemented for any data type.
    // Traits can provide default method definitions.

    // 1. derive

    // 2. returing traits with dyn
    // return a Box which contains trait
    // Box<dyn YOURTRAIT>

    // 3. Operator overloading
    // This is possible because operators are syntactic sugar for method calls.

    // 4. Drop
    // The Drop trait only has one method: drop,
    // which is called automatically when an object goes out of scope.

    // 5. Iterators
    // The Iterator trait is used to implement iterators over collections such as arrays.
    // The for construct turns some collections into iterators using the .into_iter() method.

    // 6. impl trait
    // 1) as an argument type
    // generic over a trait
    // 2) as a return type

    // 7. Clone: make a copy of the resource.
    // the default behavior is to transfer them during assignments or function calls.

    // 8. supertraits
    trait Person {
        fn name(&self) -> String;
    }
    // Person is a supertrait of Student.
    trait Student: Person {
        fn university() {}
    }

    // 9. Disambiguating overlapping traits
    // What if two traits both require the same name?
    // let val = <YOURTYPE as YOURTRAIT>::funcname();
}
