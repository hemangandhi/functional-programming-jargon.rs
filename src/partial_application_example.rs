#![doc(html_no_source)]

///Partially applying a function means creating a new function by pre-filling some of the arguments to the original function.
///
///To achieve this easily, we will be using a [partial application crate](https://crates.io/crates/partial_application)
///
///```rust
///
///fn foo(a: i32, b: i32, c: i32, d: i32, mul: i32, off: i32) -> i32 {
///    (a + b*b + c.pow(3) + d.pow(4)) * mul - off
///}
///
///let bar = partial!( foo(_, _, 10, 42, 10, 10) );
///
///assert_eq!(
///    foo(15, 15, 10, 42, 10, 10),
///    bar(15, 15)
///); // passes
///```
///
///Partial application helps create simpler functions from more complex ones by baking in data when you have it.
///Curried functions are automatically partially applied.
///
///### Further reading
///* [Partial Application in Haskell](https://wiki.haskell.org/Partial_application)
///
pub fn partial_application() {}
