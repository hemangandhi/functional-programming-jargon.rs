use crate::higher_kinded_type_example::HKT;

///An object that implements a map function which,
///while running over each value in the object to produce a new functor of the same type, adheres to two rules:
///
///### Preserves identity
///
///```
///object.map(x => x) ≍ object
///```
///
///### Composable
///
///```
///object.map(compose(f, g)) ≍ object.map(g).map(f)
///```
///
///(`f`, `g` are arbitrary functions)
///
///For example, below can be considered as a functor-like operation
///
///```rust
///let v: Vec<i32> = vec![1, 2, 3].into_iter().map(| x | x + 1).collect();
///
///assert_eq!(v, vec![2, 3, 4]); // passes while mapping the original vector and returns a new vector
///```
///
///While leveraging the [HKT implementation](#higher-kinded-type-hkt), You can define a trait that represents Functor like below
pub trait Functor<A, B>: HKT<A, B> {
    fn fmap<F>(self, f: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(A) -> B;
}

///Then use it against a type such as [Option](#https://doc.rust-lang.org/std/option/index.html) like
///and for example:
///```
///let z = Option::fmap(Some(1), |x| x + 1).fmap(|x| x + 1);
///assert_eq!(z, Some(3));
///```
impl<A, B> Functor<A, B> for Option<A> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(A) -> B,
    {
        self.map(f)
    }
}
