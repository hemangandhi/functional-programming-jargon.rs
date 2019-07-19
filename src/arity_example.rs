#![doc(html_no_source)]

///The number of arguments a function takes. From words like unary, binary, ternary, etc.
///This word has the distinction of being composed of two suffixes, "-ary" and "-ity."
///Addition, for example, takes two arguments, and so it is defined as a binary function or a function with an arity of two.
///Such a function may sometimes be called "dyadic" by people who prefer Greek roots to Latin.
///Likewise, a function that takes a variable number of arguments is called "variadic,"
///whereas a binary function must be given two and only two arguments, currying and partial application notwithstanding (see below).
///```rust
///let sum = |a: i32, b: i32| { a + b }; // The arity of sum is 2
///    let result = sum(1, 2);
///    assert_eq!(result, 3);
///```
pub fn arity() {}
