#![doc(html_no_source)]

///A closure is a scope which retains variables available to a function when it's created. This is important for
///[partial application](#partial-application) to work.
///
///```rust
///let add_to = |x: i32| move |y: i32| x + y;
///```
///
///We can call `add_to` with a number and get back a function with a baked-in `x`. Notice that we also need to move the ownership of the x to the internal lambda.
///
///```rust
///let add_to_five = add_to(5);
///```
///
///In this case the `x` is retained in `add_to_five`'s closure with the value `5`. We can then call `add_to_five` with the `y`
///and get back the desired number.
///
///```rust
///add_to_five(3); // => 8
///assert_eq!(add_to_five(3), 8);
///```
///
///Closures are commonly used in event handlers so that they still have access to variables defined in their parents when they
///are eventually called.
///
///### Further reading
///* [Lambda Vs Closure](http://stackoverflow.com/questions/220658/what-is-the-difference-between-a-closure-and-a-lambda)
///* [How do JavaScript Closures Work?](http://stackoverflow.com/questions/111102/how-do-javascript-closures-work)
pub fn closure() {}
