#![doc(html_no_source)]

/// A Sum type is the combination of two types together into another one.
/// It is called sum because the number of possible values in the result type is the sum of the input types.
///
/// Rust has `enum` that literally represent `sum` in ADT.
/// ```
///
///    enum WeakLogicValues {
///        True(bool),
///        False(bool),
///        HalfTrue(bool),
///    }
///    // WeakLogicValues = bool + otherbool + anotherbool
///
///    struct Point {
///        x: i32,
///        y: i32,
///    }
///```
pub fn adt_example() {}
