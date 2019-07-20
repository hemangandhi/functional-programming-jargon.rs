#![doc(html_no_source)]
#[macro_use]
use crate::compose_example::compose_two;

///An object with a function that "combines" that object with another of the same type.
///
///One simple monoid is the addition of numbers:
///
///```rust
///assert_eq!(1 + 1, 2);
/// // i32: 2
///```
///
///In this case number is the object and `+` is the function.
///
///An "identity" value must also exist that when combined with a value doesn't change it.
///
///The identity value for addition is `0`.
///
///```rust
///assert_eq!(1 + 0, 0);
///// i32: 1
///```
///
///It's also required that the grouping of operations will not affect the result (associativity):
///
///```rust
///assert_eq!(1 + (2 + 3)(1 + 2) + 3);
///// bool: true
///```
///
///Array concatenation also forms a monoid:
///
///```rust
///assert_eq!([vec![1, 2, 3], vec![4, 5, 6]].concat(),vec![1, 2, 3, 4, 5, 6]);
///// Vec<i32>: vec![1, 2, 3, 4, 5, 6]
///```
///
///The identity value is empty array `[]`
///
///```rust
///assert_eq!([vec![1, 2], vec![]].concat(), vec![1, 2]);
///// Vec<i32>: vec![1, 2]
///```
///
///If identity and compose functions are provided, functions themselves form a monoid:
///
///```rust
///fn identity<A>(a: A) -> A {
///    a
///}
///```
///
///`foo` is any function that takes one argument.
///
///```rust
///let id_comp = compose!(foo, identity) == foo && compose!(identity, foo) == foo;
///assert_eq!(id_comp, true);
///```
pub fn monoid_example() {}
