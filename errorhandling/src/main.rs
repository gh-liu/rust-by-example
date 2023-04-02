fn main() {
    // panic: unrecoverable errors
    // Option: a value is optional or when the lack of a value is not an error condition
    // Result: there is a chance that things do go wrong

    // 1. panic
    // It prints an error message, starts unwinding the stack, and usually exit the program.

    // 2. panic strategy: abort and unwind

    // 3. Option & unwarp
    // Option<T> is used when absence is a possibility: Some<T> None
    // explicitly handled via `match` or implicitly(return the inner element or panic) with `umwarp`
    // 3.1 question marker ?
    // If x is an Option, then evaluating x? will return the underlying value if x is Some,
    // otherwise it will terminate whatever function is being executed and return None.
    // 3.2 combinators: map (https://doc.rust-lang.org/std/option/enum.Option.html#method.map)
    // Maps an `Option<T>` to `Option<U>` by applying a function to a contained value.
    // 3.3 combinators: and_then
    // calls function input with the wrapped value and returns the result.
    // 3.4 Default: or, or_else, get_or_insert, get_or_insert_with

    // 4. Result<T, E>: Ok(T), Err(E)

    // 5. multiple error types
    // 5.1 Pulling Results out of Options: embed them in each other
    // 5.2 define error type: mask all of the different errors with a single type of error
    // 5.3 boxing errors
    // 5.4 other use of ?
    // It actually means unwrap or return Err(From::from(err)).
    // 5.5 wrap error

    // 6. iterating over results
}
