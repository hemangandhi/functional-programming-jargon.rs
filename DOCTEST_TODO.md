
# Migrating README

* [Morphism](README.md#morphism)
  * [Endomorphism](README.md#endomorphism)
  * [Isomorphism](README.md#isomorphism)
  * [Homomorphism](README.md#homomorphism)
  * [Catamorphism](README.md#catamorphism)
  * [Hylomorphism](README.md#hylomorphism)
  * [Anamorphism](README.md#anamorphism)
* [Setoid](README.md#setoid)
* [Semigroup](README.md#semigroup)
* [Foldable](README.md#foldable)
* [Lens](README.md#lens)
* [Type Signature](README.md#type-signature)
* [Algebraic data type](README.md#algebraic-data-type)
  * [Sum Type](README.md#sum-type)
  * [Product Type](README.md#product-type)

# Re-organize into topic-based modules

Makes the docs actually well-structured to search (also to manage
expectations about code samples).

There'll have to be some effort expended on actually getting the
code to still compile with import references changing.

## Smaller re-structuring

- Make `monoid.rs` a trait.

# Scripting/Linting

This is a side-goal so that, in theory, the editing workflow doesn't
change regardless of the use of doctest.

- [x] end everything jargon-related in `_example.rs`.
- [ ] get consistency with the titles in README.md (or a decent mapping)
- [ ] force linting through the python script
- [ ] look into rust reflection to figure out what code can live in doc comments and what should be kept as code that can be exported
