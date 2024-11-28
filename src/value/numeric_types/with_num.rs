use num_traits::{cast};
pub use num_traits::{
    NumCast as EvalexprNumCast,
    AsPrimitive as EvalexprAsPrimitive,
};
use crate::{EvalexprFloat, EvalexprInt, EvalexprNumericTypes, TupleType, Value};

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

/// A helper trait for easier conversion
pub trait ConvertibleWithEvalexprNumericTypes {
    /// Converts a vablue within the context
    fn into_with<V: EvalexprNumericTypesConvert>(self) -> Option<Value<V>>;
}



macro_rules! impl_convertible {
    
    (into $t: ty $(;)?) => {
        impl ConvertibleWithEvalexprNumericTypes for $t {
            fn into_with<V: EvalexprNumericTypesConvert>(self) -> Option<Value<V>> {
                Some(self.into())
            }
        }
    };
    
    (float $t: ty $(;)?) => {
        impl ConvertibleWithEvalexprNumericTypes for $t {
            fn into_with<V: EvalexprNumericTypesConvert>(self) -> Option<Value<V>> {
                Some(Value::<V>::Float(V::num_to_float(self)?))
            }
        }
    };
    
    (int $t: ty $(;)?) => {
        impl ConvertibleWithEvalexprNumericTypes for $t {
            fn into_with<V: EvalexprNumericTypesConvert>(self) -> Option<Value<V>> {
                Some(Value::<V>::Int(V::num_to_int(self)?))
            }
        }
    };
    
    (into $t: ty; $($tt:tt)*) => {
        impl_convertible!(into $t);
        impl_convertible!($($tt)*);
    };
    
    (float $t: ty; $($tt:tt)*) => {
        impl_convertible!(float $t);
        impl_convertible!($($tt)*);
    };
    
    (int $t: ty; $($tt:tt)*) => {
        impl_convertible!(int $t);
        impl_convertible!($($tt)*);
    };
}

impl<NumericType: EvalexprNumericTypesConvert> ConvertibleWithEvalexprNumericTypes for TupleType<NumericType> {
    fn into_with<V: EvalexprNumericTypesConvert>(self) -> Option<Value<V>> {
        Some(
            Value::Tuple(
                self.into_iter()
                    .map(|value| value.into_with::<V>())
                    .collect::<Option<Vec<_>>>()?
            )
        )
    }
}

impl<NumericType: EvalexprNumericTypesConvert> ConvertibleWithEvalexprNumericTypes for Value<NumericType> {
    fn into_with<V: EvalexprNumericTypesConvert>(self) -> Option<Value<V>> {
        match self {
            Value::String(value) => {
                value.into_with()
            }
            Value::Float(value) => {
                Some(Value::Float(V::num_to_float(value)?))
            }
            Value::Int(value) => {
                Some(Value::Int(V::num_to_int(value)?))
            }
            Value::Boolean(value) => {
                value.into_with()
            }
            Value::Tuple(value) => {
                value.into_with()
            }
            Value::Empty => {
                Some(Value::Empty)
            }
        }
    }
}


impl_convertible! {
    into String;
    into bool;
    into ();
    into &str;
    int u8;
    int i8;
    int u16;
    int i16;
    int u32;
    int i32;
    int u64;
    int i64;
    int u128;
    int i128;
    int usize;
    int isize;
    float f32;
    float f64;
}