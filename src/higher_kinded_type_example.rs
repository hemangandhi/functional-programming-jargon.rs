///Rust does not support Higher Kinded Types [yet](https://github.com/rust-lang/rust/issues/8922). First of all, HKT is a
///type with a "hole" in it, so you can declare a type signature such as `trait Functor<F<A>>`.
///
///Although Rust lacks in a native support for HKT, we always have a walk around called [Lightweight Higher Kinded Type](https://www.cl.cam.ac.uk/~jdy22/papers/lightweight-higher-kinded-polymorphism.pdf)
///
///An implementation example of above theory in Rust would look like below:
pub trait HKT<A, B> {
    type URI;
    type Target;
}

/// Lifted Option
/// So that it can be seen as a higher kinded type
impl<A, B> HKT<A, B> for Option<A> {
    type URI = Self;
    type Target = Option<B>;
}
