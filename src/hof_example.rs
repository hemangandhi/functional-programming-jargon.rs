#![doc(html_no_source)]

///A function which takes a function as an argument and/or returns a function.
///
///```rust
///let filter = | predicate: fn(&i32) -> bool, xs: Vec<i32> | {
///    xs.into_iter().filter(predicate).collect::<Vec<i32>>()
///};
///```
///
///```rust
///let is_even = |x: &i32| { x % 2 == 0 };
///```
///
///```rust
///filter(is_even, vec![1, 2, 3, 4, 5, 6]);
///assert_eq!(result, vec![2, 4, 6]);
///```
pub fn higher_order_function() {}
