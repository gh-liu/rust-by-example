fn main() {
    // macros are expanded into source code that gets compiled with the rest of the program.
    // Unlike macros in C and other languages, Rust macros are expanded into abstrac syntax trees,
    // rather than string preprocessing.
    // 1) DRY 2)DSL 3)Variadic interfaces

    // 1. syntax
    // 1.1 designators:
    // arguments of a macro are prefixe by a dollar sign $,
    // and type annotated with a designator.
    // [avaiable designators](https://doc.rust-lang.org/reference/macros-by-example.html):
    // block
    // expr is used for expressions
    // ident is used for variable/function names
    // item
    // literal is used for literal constants
    // pat (pattern)
    // path
    // stmt (statement)
    // tt (token tree)
    // ty (type)
    // vis (visibility qualifier)

    // 1.2 overload
    // work similarly to a match block

    //1.3 repeat
    // +, *
    // $(...)+

    // 2. DRY

    // 3. DSL
}
