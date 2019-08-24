use crate::hkt::HKT;
use std::iter::FromIterator;

pub trait Empty {
    fn empty() -> Self;
}

macro_rules! impl_empty_i {
    ($($t:ty)*) => ($(
        impl Empty for $t {
            fn empty() -> Self {
                0
            }
        }
    )*)
}

//impl_empty_i! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! impl_empty_f {
    ($($t:ty)*) => ($(
        impl Empty for $t {
            fn empty() -> Self {
                0.0
            }
        }
    )*)
}

//impl_empty_f! { f32 f64 }

impl<T> Empty for T
where
    // Big-time hack: <T as HKT<T>>::Current yanks out a generic
    // argument and T: HKT<T> is a reasonable bound for any HKT.
    T: HKT<T> + FromIterator<<T as HKT<T>>::Current>,
{
    fn empty() -> Self {
        std::iter::empty()
    }
}
