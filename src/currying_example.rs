#![doc(html_no_source)]

///The process of converting a function that takes multiple arguments into a function that takes them one at a time.
///
///Each time the function is called it only accepts one argument and returns a function that takes one argument until all arguments are passed.
///
///
///```rust
///fn add(x: i32) -> impl Fn(i32)-> i32 {
///    move |y| x + y
///}
///
///let add5 = add(5);
///add5(10); // 15
///assert_eq!(result, 15);
///```
///
///### Further reading
///* [Currying in Rust](https://hashnode.com/post/currying-in-rust-cjpfb0i2z00cm56s2aideuo4z)
pub fn currying() {}
