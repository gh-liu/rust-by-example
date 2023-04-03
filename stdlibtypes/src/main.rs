fn main() {
    // growable string
    // growable vector
    // optional type
    // error handing type
    // heap alloated pointer

    // 1. Box: stack and heap
    // all value in rust are stack alloated by default;
    // value can be boxed(alloated on the heap) by creating a Box<T>;
    // a box is a smart pointer to a heap alloated value of type T.
    // when a box goes out of scope, its destructor is called, the inner object is destroyed,
    // and the memory is freed.
    // boxed value can be dereferenced using the * operator(copy the inner value).

    // 2. Vector: resizable arrays
    // 1) pointer to the data, 2) length 3) capacity

    // 3. String
    // 1) String is stored as a vector of bytes(Vec(u8)), which is heap alloated.
    // 2) &str is a slice(&[u8])

    // 4. Option<T>
    // None:
    // Some(T):

    // 5. Result<T, E>
    // Ok(T):
    // Err(E):
    // 5.1
    // ? is used at the end of an expression returning a Result, is equivalent to a match expression:
    // Err(E) branch expands to an early `return Err(From::from(err))`;
    // Ok(T) branch expands to an ok expression.

    // 6. panic!

    // 7. HashMap<K, V>: growable and shrinkable
    // key can be any type that implements the Eq and the Hash traits;
    // 7.2 HashSet<T>: just care about the keys (HashMap(K,())), which is guaranteed to not have duplicate elements.
    // sets operation: union, difference, interserction, symmetric_difference

    // 8. RC: Reference Counting
    // Rc keeps track of the number of the references which means the number of owners of the value wrapped inside an Rc.
    // Reference count of an Rc increases by 1 whenever an Rc is cloned,
    // and decreases by 1 whenever one cloned Rc is dropped out of the scope.

    // 9. Arc: Atomically Reference Counted
    // shared ownership between threads.
}
