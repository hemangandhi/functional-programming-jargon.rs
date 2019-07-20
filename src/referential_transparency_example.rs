#![doc(html_no_source)]

///An expression that can be replaced with its value without changing the behavior of the program is said to be referentially transparent.
///
///Say we have function greet:
///
///```rust
///let greet = || "Hello World!";
///let msg = greet();
///
///assert_eq!(msg, "Hello World!");
///```
///
///Any invocation of `greet()` can be replaced with `Hello World!` hence greet is referentially transparent.
pub fn referential_transparency() {}
