use crate::functor_example::Functor;
use crate::higher_kinded_type_example::HKT;

///Extend runs a function on the Comonad.
trait Extend<A, B>: Functor<A, B> + Sized {
    fn extend<W>(self, f: W) -> <Self as HKT<A, B>>::Target
    where
        W: FnOnce(Self) -> B;
}

impl<A, B> Extend<A, B> for Option<A> {
    fn extend<W>(self, f: W) -> Self::Target
    where
        W: FnOnce(Self) -> B,
    {
        self.map(|x| f(Some(x)))
    }
}

///Extract takes a value out of a functor.
trait Extract<A> {
    fn extract(self) -> A;
}

impl<A> Extract<A> for Option<A> {
    fn extract(self) -> A {
        self.unwrap() // is there a better way to achieve this?
    }
}

///An object that has `extract` and `extend` functions.
trait Comonad<A, B>: Extend<A, B> + Extract<A> {}

#[test]
fn comonad_test() {
    let z = Some(1).extend(|x| x.extract() + 1);
    assert_eq!(z, Some(2));
}
