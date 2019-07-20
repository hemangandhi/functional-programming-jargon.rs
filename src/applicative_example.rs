use crate::functor_example::Functor;
use crate::higher_kinded_type_example::HKT;

///In order to achieve
///this, we will introduce another [higher kinded type](#higher-kinded-type-hkt), called `HKT3` that is capable of doing this.
pub trait HKT3<A, B, C> {
    type Target2;
}

///For this example, we will use Option datatype.
impl<A, B, C> HKT3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
}

///Since Applicative implements Apply for `ap`  and `Pure` for `of` according to [Fantasy Land specification](https://github.com/fantasyland/fantasy-land#applicative)
///we must implement the types like below:
pub trait Apply<A, F, B>: Functor<A, B> + HKT3<A, F, B>
where
    F: FnOnce(A) -> B,
{
    fn ap(self, f: <Self as HKT3<A, F, B>>::Target2) -> <Self as HKT<A, B>>::Target;
}

impl<A, F, B> Apply<A, F, B> for Option<A>
where
    F: FnOnce(A) -> B,
{
    fn ap(self, f: Self::Target2) -> Self::Target {
        self.and_then(|v| f.map(|z| z(v)))
    }
}

///Since Applicative implements Apply for `ap`  and `Pure` for `of` according to [Fantasy Land specification](https://github.com/fantasyland/fantasy-land#applicative)
///we must implement the types like below:
pub trait Pure<A>: HKT<A, A> {
    fn of(self) -> <Self as HKT<A, A>>::Target;
}

impl<A> Pure<A> for Option<A> {
    fn of(self) -> Self::Target {
        self
    }
}

///An applicative functor is an object with an `ap` function. `ap` applies a function in the object to a value in another
///object of the same type. Given a pure program `g: (b: A) -> B`, we must lift it to `g: (fb: F<A>) -> F<B>`.
pub trait Applicative<A, F, B>: Apply<A, F, B> + Pure<A>
where
    F: FnOnce(A) -> B,
{
}

///Then we can use Option Applicative like this:
///```rust
///let x = Option::of(Some(1)).ap(Some(|x| x + 1));
///assert_eq!(x, Some(2));
///```
impl<A, F, B> Applicative<A, F, B> for Option<A> where F: FnOnce(A) -> B {}
