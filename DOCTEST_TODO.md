
# Migrating README

* [Higher Kinded Type](#higher-kinded-type-hkt) <- should be in own file, so `functor.rs` will have to be fixed.
* [Functor](#functor)
* [Pointed Functor](#pointed-functor)
* [Lifting](#lifting)
* [Equational Reasoning](#equational-reasoning)
* [Monoid](#monoid)
* [Monad](#monad)
* [Comonad](#comonad)
* [Applicative](#applicative)
* [Morphism](#morphism)
  * [Endomorphism](#endomorphism)
  * [Isomorphism](#isomorphism)
  * [Homomorphism](#homomorphism)
  * [Catamorphism](#catamorphism)
  * [Hylomorphism](#hylomorphism)
  * [Anamorphism](#anamorphism)
* [Setoid](#setoid)
* [Semigroup](#semigroup)
* [Foldable](#foldable)
* [Lens](#lens)
* [Type Signature](#type-signature)
* [Algebraic data type](#algebraic-data-type)
  * [Sum Type](#sum-type)
  * [Product Type](#product-type)

# Re-organize into topic-based modules

Makes the docs actually well-structured to search (also to manage
expectations about code samples).

There'll have to be some effort expended on actually getting the
code to still compile with import references changing.

# Scripting/Linting

This is a side-goal so that, in theory, the editing workflow doesn't
change regardless of the use of doctest.

- [x] end everything jargon-related in `_example.rs`.
- [ ] get consistency with the titles in README.md (or a decent mapping)
- [ ] force linting through the python script
- [ ] look into rust reflection to figure out what code can live in doc comments and what should be kept as code that can be exported
