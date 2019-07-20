#![doc(html_no_source)]
use std::time::SystemTime;

///A function or expression is said to have a side effect if apart from returning a value,
///it interacts with (reads from or writes to) external mutable state.
///
///```rust
///use std::time::SystemTime;
///
///let now = SystemTime::now();
///```
///
///```rust
///println!("IO is a side effect!");
///// IO is a side effect!
///```
pub fn side_effects() {}
