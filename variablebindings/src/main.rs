fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer {:?}", copied_integer);
    println!("A boolean {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // mutablity
    let _immutable_bindling = 1;
    let mut mutable_bindling = 1;

    println!("Before mutation: {}", mutable_bindling);
    mutable_bindling = 2;
    println!("After mutation: {}", mutable_bindling);

    // ERROR
    // _immutable_bindling = 2;

    // scope and shadawing
    // variable binding hava a scope, and are constrained to live in a block.
    let long_lived_binding = 1;
    let shadawing_binding = 11;
    println!("before being shadowed : {}", shadawing_binding);
    // this is a block, has a smaller scope.
    {
        let short_lived_bind = 2;
        println!("{}", short_lived_bind);

        let shadawing_binding = 22;
        println!("shadowed in inner block: {}", shadawing_binding);
    }
    // end of the block

    println!("outer long: {}", long_lived_binding);

    let shadawing_binding = 30;
    println!("shadowed in outter block: {}", shadawing_binding);

    // declare first
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding {}", a_binding);

    // freezing
    // when mutable data bound by the same name immutably, it alse freezs.
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
    }
}
