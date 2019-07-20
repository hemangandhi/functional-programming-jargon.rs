use crate::applicative_example::{Applicative, Pure};
use crate::higher_kinded_type_example::HKT;

///`chain` is like `map` except it un-nests the resulting nested object.
///First, `Chain` type can be implemented like below:
pub trait Chain<A, B>: HKT<A, B> {
    fn chain<F>(self, f: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(A) -> <Self as HKT<A, B>>::Target;
}

impl<A, B> Chain<A, B> for Option<A> {
    fn chain<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(A) -> <Self as HKT<A, B>>::Target,
    {
        self.and_then(f)
    }
}

///A [Monad](https://github.com/fantasyland/fantasy-land#monad) is a trait that implements `Applicative` and `Chain` specifications. ///
///Then `Monad` itself can simply derive `Chain` and `Applicative`
///
///```
///let x = Option::of(Some(1)).chain(|x| Some(x + 1));
///assert_eq!(x, Some(2));
///```
///
///`pure` is also known as `return` in other functional languages. `flat_map` is also known as `bind` in other languages.
pub trait Monad<A, F, B>: Chain<A, B> + Applicative<A, F, B>
where
    F: FnOnce(A) -> B,
{
}

impl<A, F, B> Monad<A, F, B> for Option<A> where F: FnOnce(A) -> B {}
