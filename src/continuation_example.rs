#![doc(html_no_source)]

///At any given point in a program, the part of the code that's yet to be executed is known as a continuation.
///
///```rust
///let print_as_string = |num: i32| println!("Given {}", num);
///
///let add_one_and_continue = |num: i32, cc: fn(i32)| {
///    let result = num + 1;
///    cc(result)
///};
///
///add_one_and_continue(1, print_as_string); // Given 2
///```
///
///Continuations are often seen in asynchronous programming when the program needs to wait to receive data before it can continue.
/// The response is often passed off to the rest of the program, which is the continuation, once it's been received.
pub fn continuation() {}
