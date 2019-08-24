use std::collections::HashMap;

// TODO: use a declarative macro (see https://github.com/rust-lang/rust/issues/39412) to make this
// one macro that is invoked repeatedly.

pub trait HKT<U> {
    type Current;
    type Target;
}

macro_rules! derive_hkt {
    ($t: ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type Current = T;
            type Target = $t<U>;
        }
    };
}

impl<A, B, I, F, I2> HKT<B> for F
where
    F: IntoIterator<Item = A, IntoIter = I>,
    I: IntoIterator<Item = B, IntoIter = I2> + Iterator<Item = A>,
    I2: Iterator<Item = B>,
{
    type Current = A;
    type Target = I2;
}

pub trait HKT2<U1, U2> {
    type Current1;
    type Current2;
    type Target;
}

impl<U1, U2, T1, T2, T3> HKT<U2> for HKT2<U1, U2, Current1 = T1, Current2 = T2, Target = T3> {
    type Current = <Self as HKT2<U1, U2>>::Current1; // Technically T1, but this emphasizes the intent
    type Target = <Self as HKT2<U1, U2>>::Target; // Technically T3, but this emphasizes the intent
}

macro_rules! derive_hkt2 {
    ($t:ident) => {
        impl<T1, T2, U1, U2> HKT2<U1, U2> for $t<T1, T2> {
            // The currently contained types
            type Current1 = T1;
            type Current2 = T2;
            // How the U's get filled in.
            type Target = $t<U1, U2>;
        }
    };
}

derive_hkt2!(HashMap);
