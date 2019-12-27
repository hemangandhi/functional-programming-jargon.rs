use crate::applicative::Applicative;
use crate::chain::Chain;

pub trait Monad<A, B>: Chain<B> + Applicative<A, B> {}

impl<A, B> Monad<A, B> for Option<A> {}

impl<A, B, E> Monad<A, B> for Result<A, E> {}

macro_rules! mon_do {
    ($e: expr; $($rest:tt)*) => $e.chain(|_| mon_do!($($rest)*)),
    (return<$hkt:ty of $s:ty> $e:expr;) => <$t<$s> as Pure<$s>>::of($e),

}
