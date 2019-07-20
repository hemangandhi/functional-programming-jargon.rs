#![doc(html_no_source)]
///A contract specifies the obligations and guarantees of the behavior from a function or expression at runtime.
///This acts as a set of rules that are expected from the input and output of a function or expression,
///and errors are generally reported whenever a contract is violated.
///
///```rust
///let contract = | x: &i32 | -> bool {
///    x > &10
///};
///
///let add_one = | x: &i32 | -> Result<i32, String> {
///    if contract(x) {
///        return Ok(x + 1);
///    }
///    Err("Cannot add one".to_string())
///};
///```
///
///Then you can use `add_one` like
///
///```rust
///let expected = 12;
///match add_one(&11) {
///    Ok(x) => assert_eq!(x, expected),
///    _ => panic!("Failed!")
///}
///```
pub fn contracts_example() {}
