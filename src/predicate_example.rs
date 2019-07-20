#![doc(html_no_source)]
///A predicate is a function that returns true or false for a given value.
///A common use of a predicate is as the callback for array filter.
///
///```rust
///let predicate = | a: &i32 | a.clone() > 2;
///
///let result = (vec![1, 2, 3, 4]).into_iter()
///                               .filter(predicate)
///                               .collect::<Vec<i32>>();
///
///assert_eq!(result, vec![3, 4]); // passes
///```
pub fn predicate_example() {}
