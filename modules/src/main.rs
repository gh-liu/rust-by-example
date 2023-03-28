// a module is a collection of items: functions, structs, traits, impl blocks,
// and even other modules.
// By default, the items in a module have private visibility,
// but this can be overridden with the pub modifier.
mod my_mod {
    fn private_func() -> () {
        ()
    }

    pub fn public_func() -> () {
        ()
    }
}

fn main() {
    my_mod::public_func();

    // The `use` declaration can be used to bind a full path to a new name, for easier access;
    // You can use the `as` keyword to bind imports to a different name.

    // `super, self` keyword
    // The `self` keyword refers to the current module scope
    // The `super` keyword refers to the parent scope

    // file hierarchy
    // modules can be mapped to a file/directory hierarchy.
}
