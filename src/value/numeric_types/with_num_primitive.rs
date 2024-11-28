use std::fmt::{Debug, Display};
use std::ops::{Shl, Shr};
use std::str::FromStr;
use num_traits::{CheckedNeg, CheckedRem, Float, Pow, PrimInt, Signed};
use crate::{EvalexprError, EvalexprNumericTypes, EvalexprResult};

/// An integer type that can be used by `evalexpr`.
pub trait EvalexprInt<NumericTypes: EvalexprNumericTypes<Int = Self>>:
PrimInt
+ Debug
+ Display
+ FromStr
+ CheckedRem
+ CheckedNeg
+ Shl<Self, Output = Self>
+ Shr<Self, Output = Self>
+ Send
+ Sync
{
    /// Convert `self` into [`usize`].
    fn into_usize(self) -> EvalexprResult<usize, NumericTypes> {
        num_traits::cast(self).ok_or_else(|| EvalexprError::IntIntoUsize {
            int: self,
        })
    }

    /// Convert a value of type [`usize`] into `Self`.
    fn from_usize(int: usize) -> EvalexprResult<Self, NumericTypes> {
        num_traits::cast(int).ok_or_else(|| EvalexprError::IntFromUsize {
            usize_int: int,
        })
    }

    /// Parse `Self` from a hex string.
    #[expect(clippy::result_unit_err)]
    fn from_hex_str(literal: &str) -> Result<Self, ()> {
        Self::from_str_radix(literal, 16).map_err(|_| ())
    }

    /// Compute the absolute value, returning an error on overflow.
    fn abs(&self) -> EvalexprResult<Self, NumericTypes>;
}

/// A float type that can be used by `evalexpr`.
pub trait EvalexprFloat<NumericTypes: EvalexprNumericTypes<Float = Self>>:
Float
+ Debug
+ Display
+ Pow<Self, Output = Self>
+ FromStr
+ Send
+ Sync
{
    /// Returns the absolute value of self.
    fn abs(&self) -> Self;

    /// Generate a random float value between 0.0 and 1.0.
    ///
    /// If the feature `rand` is not enabled, then this method always returns [`EvalexprError::RandNotEnabled`].
    fn random() -> EvalexprResult<Self, NumericTypes>;
}





macro_rules! impl_defaults {
    (unsigned: $($ty:ident),+ $(,)?) => {
        $(
        impl<NumericTypes: EvalexprNumericTypes<Int = $ty>> EvalexprInt<NumericTypes> for $ty
        {
            fn abs(&self) -> EvalexprResult<Self, NumericTypes> {
                Ok(*self)
            }
        }
        )+
    };
    (signed: $($ty:ident),+ $(,)?) => {
        $(
        impl<NumericTypes: EvalexprNumericTypes<Int = $ty>> EvalexprInt<NumericTypes> for $ty
        {
            fn abs(&self) -> EvalexprResult<Self, NumericTypes> {
                Ok($ty::abs(*self))
            }
        }
        )+
    };
    (float: $($ty:ident),+ $(,)?) => {
        $(
        impl<NumericTypes: EvalexprNumericTypes<Float = $ty>> EvalexprFloat<NumericTypes> for $ty
        {
            fn abs(&self) -> Self {
                Signed::abs(self)
            }
         
            fn random() -> EvalexprResult<Self, NumericTypes> {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "rand")] {
                        Ok(rand::random())
                    } else {
                        Err(EvalexprError::RandNotEnabled)
                    }
                }
            }
        }
        )+
    };
    ($($tt:tt: $($ty:ident),+;)+) => {
        $(
        impl_defaults!($tt: $($ty),+);
        )+
    };
}

impl_defaults! {
    float: f64, f32;
    signed: i8, i16, i32, i64, i128, isize;
    unsigned: u8, u16, u32, u64, u128, usize;
}




