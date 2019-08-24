use crate::hkt::HKT;
use std::iter::FromIterator;
use std::ops::Add;

pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

/*
impl<I> Semigroup for I
where
    I: Add<I, Output = I>,
{
    fn combine(self, other: I) -> I {
        self.add(other)
    }
}
*/

impl<T, I> Semigroup for T
where
    T: HKT<T>
        + FromIterator<<T as HKT<T>>::Current>
        + IntoIterator<Item = <T as HKT<T>>::Current, IntoIter = I>,
{
    fn combine(self, other: Self) {
        self.into_iter().extend(other.into_iter()).collect()
    }
}
