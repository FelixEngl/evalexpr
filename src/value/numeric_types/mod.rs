
use std::fmt::Debug;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "num_primitive")] {
        mod with_num_primitive;
        pub use with_num_primitive::*;
    } else {
        mod without_num;
        pub use without_num::*;
    }
}

cfg_if! {
    if #[cfg(feature = "num")] {
        pub mod with_num;
    }
}


/// A trait to parameterise `evalexpr` with an int type and a float type.
///
/// See [`EvalexprInt`] and [`EvalexprFloat`] for the requirements on the types.
pub trait EvalexprNumericTypes: 'static + Sized + Debug + Clone + PartialEq + Sync + Send {
    /// The integer type.
    #[cfg(feature = "serde")]
    type Int: EvalexprInt<Self> + serde::Serialize + for<'de> serde::Deserialize<'de>;

    /// The integer type.
    #[cfg(not(feature = "serde"))]
    type Int: EvalexprInt<Self>;

    /// The float type.
    #[cfg(feature = "serde")]
    type Float: EvalexprFloat<Self> + serde::Serialize + for<'de> serde::Deserialize<'de>;

    /// The float type.
    #[cfg(not(feature = "serde"))]
    type Float: EvalexprFloat<Self>;

    /// Convert an integer to a float using the `as` operator or a similar mechanic.
    fn int_as_float(int: &Self::Int) -> Self::Float;

    /// Convert a float to an integer using the `as` operator or a similar mechanic.
    fn float_as_int(float: &Self::Float) -> Self::Int;
}

/// See [`EvalexprNumericTypes`].
///
/// This empty struct uses [`i64`] as its integer type and [`f64`] as its float type.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DefaultNumericTypes;

impl EvalexprNumericTypes for DefaultNumericTypes {
    type Int = i64;
    type Float = f64;

    fn int_as_float(int: &Self::Int) -> Self::Float {
        *int as Self::Float
    }

    fn float_as_int(float: &Self::Float) -> Self::Int {
        *float as Self::Int
    }
}

