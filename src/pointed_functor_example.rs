#[derive(Debug, PartialEq, Eq)]
/// A more Haskell-like `Option`, so that we can implement
/// a pointed functor.
enum Maybe<T> {
    Nothing,
    Just(T),
}

///An object with an of function that puts any single value into it.
///Then use it like
///
///```rust
///let pointed_functor = Maybe::of(1);
///
///assert_eq!(pointed_functor, Maybe::Just(1));
///```
impl<T> Maybe<T> {
    fn of(x: T) -> Self {
        Maybe::Just(x)
    }
}
