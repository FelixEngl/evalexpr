use num_traits::{cast};
pub use num_traits::{
    NumCast as EvalexprNumCast,
    AsPrimitive as EvalexprAsPrimitive,
};
use crate::{EvalexprFloat, EvalexprInt, EvalexprNumericTypes};

/// Marks a numeric int type as copyable
pub trait EvalexprNumericTypesWithCopyInt:EvalexprNumericTypes<Int=<Self as EvalexprNumericTypesWithCopyInt>::IntCopy> {
    /// The integer type.
    #[cfg(feature = "serde")]
    type IntCopy: EvalexprInt<Self> + Copy + serde::Serialize + for<'de> serde::Deserialize<'de>;

    /// The integer type.
    #[cfg(not(feature = "serde"))]
    type IntCopy: EvalexprInt<Self> + Copy;
}

impl<T> EvalexprNumericTypesWithCopyInt for T where T: EvalexprNumericTypes, T::Int: Copy {
    type IntCopy = T::Int;
}

/// Marks a numeric float type as copyable
pub trait EvalexprNumericTypesWithCopyFloat:EvalexprNumericTypes<Float=<Self as EvalexprNumericTypesWithCopyFloat>::FloatCopy> {

    /// The float type.
    #[cfg(feature = "serde")]
    type FloatCopy: EvalexprFloat<Self> + serde::Serialize + for<'de> serde::Deserialize<'de> + Copy;

    /// The float type.
    #[cfg(not(feature = "serde"))]
    type FloatCopy: EvalexprFloat<Self> + Copy;
}

impl<T> EvalexprNumericTypesWithCopyFloat for T where T: EvalexprNumericTypes, T::Float: Copy {
    type FloatCopy = T::Float;
}

/// Marks a numeric type as copyable
pub trait EvalexprNumericTypesWithCopy: EvalexprNumericTypesWithCopyFloat + EvalexprNumericTypesWithCopyInt {}

impl<T> EvalexprNumericTypesWithCopy for T where T: EvalexprNumericTypesWithCopyFloat + EvalexprNumericTypesWithCopyInt {
}

/// An extension to `EvalexprNumericTypes` to allow int conversions.
pub trait EvalexprNumericTypesIntConvert: EvalexprNumericTypes<Int=<Self as EvalexprNumericTypesIntConvert>::IntConvert> {
    /// The integer type.
    #[cfg(feature = "serde")]
    type IntConvert: EvalexprInt<Self> + EvalexprNumCast + serde::Serialize + for<'de> serde::Deserialize<'de>;

    /// The integer type.
    #[cfg(not(feature = "serde"))]
    type IntConvert: EvalexprInt<Self> + EvalexprNumCast;

    /// Converts a number to an int
    fn num_to_int<T: EvalexprNumCast>(num: T) -> Option<Self::Int> {
        cast(num)
    }
    
    /// Converts an int to a number
    fn int_to_num<T: EvalexprNumCast>(int: Self::Int) -> Option<T> {
        cast(int)
    }
}

impl<T> EvalexprNumericTypesIntConvert for T
where
    T: EvalexprNumericTypes,
    T::Int: EvalexprNumCast,
{
    type IntConvert = T::Int;
}

/// An extension to `EvalexprNumericTypes` to allow float conversions.
pub trait EvalexprNumericTypesFloatConvert: EvalexprNumericTypes<Float=<Self as EvalexprNumericTypesFloatConvert>::FloatConvert> {
    /// The float type.
    #[cfg(feature = "serde")]
    type FloatConvert: EvalexprFloat<Self> + EvalexprNumCast + serde::Serialize + for<'de> serde::Deserialize<'de>;

    /// The float type.
    #[cfg(not(feature = "serde"))]
    type FloatConvert: EvalexprFloat<Self> + EvalexprNumCast;
    
    /// Converts a number to a float
    fn num_to_float<T: EvalexprNumCast>(num: T) -> Option<Self::Float>{
        cast(num)
    }

    /// Converts a float to a number
    fn float_to_num<T: EvalexprNumCast>(float: Self::Float) -> Option<T> {
        cast(float)
    }
}

impl<T> EvalexprNumericTypesFloatConvert for T
where
    T: EvalexprNumericTypes,
    T::Float: EvalexprNumCast,
{
    type FloatConvert = T::Float;
}


/// An extension to `EvalexprNumericTypes` to allow casts in both directions.
pub trait EvalexprNumericTypesConvert: EvalexprNumericTypesIntConvert + EvalexprNumericTypesFloatConvert {}

impl<T> EvalexprNumericTypesConvert for T 
where
    T: EvalexprNumericTypes,
    T::Int: EvalexprNumCast,
    T::Float: EvalexprNumCast
{}


/// An extension to `EvalexprNumericTypes` to allow as conversions of Int.
pub trait EvalexprNumericTypesIntCast<NumericType>: EvalexprNumericTypes<Int=Self::IntAs> + EvalexprNumericTypesWithCopyInt<IntCopy=Self::IntAs>
where
    NumericType: EvalexprNumericTypesWithCopyInt,
{
    /// The integer type.
    #[cfg(feature = "serde")]
    type IntAs: EvalexprInt<Self> + EvalexprAsPrimitive<NumericType::Int> + Copy + serde::Serialize + for<'de> serde::Deserialize<'de>;

    /// The integer type.
    #[cfg(not(feature = "serde"))]
    type IntAs: EvalexprInt<Self> + EvalexprAsPrimitive<T> + Copy;

    /// Casts an `int` to `T`
    fn cast_int_to(int: Self::IntAs) -> NumericType::Int {
        int.as_()
    }
}

/// An extension to `EvalexprNumericTypes` to allow as conversions of Int.
pub trait EvalexprNumericTypesFloatCast<NumericType>: EvalexprNumericTypes<Float=Self::FloatAs> + EvalexprNumericTypesWithCopyFloat<FloatCopy=Self::FloatAs>
where
    NumericType: EvalexprNumericTypesWithCopyFloat,
{
    /// The integer type.
    #[cfg(feature = "serde")]
    type FloatAs: EvalexprFloat<Self> + EvalexprAsPrimitive<NumericType::Float> + Copy + serde::Serialize + for<'de> serde::Deserialize<'de>;

    /// The integer type.
    #[cfg(not(feature = "serde"))]
    type FloatAs: EvalexprFloat<Self> + EvalexprAsPrimitive<T> + Copy;

    /// Casts an `int` to `T`
    fn cast_float_to(int: Self::FloatAs) -> NumericType::Float {
        int.as_()
    }
}

/// An extension to `EvalexprNumericTypes` to allow casts in both directions.
pub trait EvalexprNumericTypesCast<NumericType>: EvalexprNumericTypesIntCast<NumericType> + EvalexprNumericTypesFloatCast<NumericType> 
where
    NumericType: EvalexprNumericTypesWithCopy
{}

impl<T, O> EvalexprNumericTypesCast<O> for T
where
    T: EvalexprNumericTypesWithCopy + EvalexprNumericTypesIntCast<O> + EvalexprNumericTypesFloatCast<O>,

    O: EvalexprNumericTypesWithCopy,
{}
