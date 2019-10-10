use crate::hkt::HKT;

pub trait Pure<A>: HKT<A> {
    fn of(c: Self::Current) -> Self::Target;
}

impl<A, B> Pure<A> for Option<B> {
    fn of(a: A) -> Self::Target {
        Some(a)
    }
}

impl<A, B> Pure<A> for Vec<B> {
    fn of(a: A) -> Self::Target {
        vec![a]
    }
}

impl<A, E> Pure<A> for Result<A, E> {
    fn of(a: A) -> Self::Target {
        Ok(a)
    }
}
