#![doc(html_no_source)]
///A function is pure if the return value is only determined by its input values, and does not produce side effects.
///
///```rust
///let greet = |name: &str| { format!("Hi! {}", name) };
///
///greet("Jason"); // Hi! Jason
///```
///
///As opposed to each of the following:
///
///```rust
///let name = "Jason";
///
///let greet = || -> String {
///    format!("Hi! {}", name)
///};
///
///greet(); // String = "Hi! Jason"
///assert_eq!("Hi! Jason", greet("Jason"));
///```
///
///The above example's output is based on data stored outside of the function...
///
///```rust
///let mut greeting: String = "".to_string();
///
///let mut greet = |name: &str| {
///    greeting = format!("Hi! {}", name);
///};
///
///greet("Jason");
///
///assert_eq!("Hi! Jason", greeting); // Passes
///```
///
///... and this one modifies state outside of the function.
pub fn purity() {}
