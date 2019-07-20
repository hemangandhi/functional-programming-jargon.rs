#![doc(html_no_source)]

///The act of putting two functions together to form a third function where the output of one function is the input of the other.
///Below is an example of a compose macro is Rust, usable as follows
///
///```rust
///let add = | x: i32 | x + 2;
///let multiply = | x: i32 | x * 2;
///let divide = | x: i32 | x / 2;
///
///let intermediate = compose!(add, multiply, divide);
///
///let subtract = | x: i32 | x - 1;
///
///let finally = compose!(intermediate, subtract);
///
///let expected = 11;
///let result = finally(10);
///assert_eq!(result, expected); // passes
///```
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

///Here is a more direct function.
fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}
