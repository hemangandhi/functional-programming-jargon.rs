#![doc(html_no_source)]
use std::num;

///A function is idempotent if reapplying it to its result does not produce a different result.
///
///```rust
///// Custom immutable sort method
///let sort = | x: Vec<i32> | -> Vec<i32> {
///    let mut cloned_x = x.clone();
///    cloned_x.sort();
///    cloned_x
///};
///```
///
///Then we can use the sort method like
///
///```rust
///let x = vec![2 ,1];
///let sorted_x = sort(sort(x.clone()));
///let expected = vec![1, 2];
///assert_eq!(sorted_x, expected); // passes
///```
///
///```rust
///let abs = | x: i32 | -> i32 {
///    x.abs()
///};
///
///let x: i32 = 10;
///let result = abs(abs(x));
///assert_eq!(result, x); // passes
///```
pub fn idempotent_example() {}
