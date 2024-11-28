use std::fmt::{Debug, Display};
use std::ops::{Shl, Shr};
use std::str::FromStr;
use cfg_if::cfg_if;
use num_traits::{CheckedNeg, CheckedRem, Float, Pow, PrimInt};
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
    /// The minimum value of the integer type.
    const MIN: Self;

    /// The maximum value of the integer type.
    const MAX: Self;
    
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
    /// The smallest non-NaN floating point value.
    ///
    /// Typically, this is negative infinity.
    const MIN: Self;

    /// The largest non-NaN floating point value.
    ///
    /// Typically, this is positive infinity.
    const MAX: Self;

    /// Returns the absolute value of self.
    fn abs(&self) -> Self;

    /// Generate a random float value between 0.0 and 1.0.
    ///
    /// If the feature `rand` is not enabled, then this method always returns [`EvalexprError::RandNotEnabled`].
    fn random() -> EvalexprResult<Self, NumericTypes>;
}

impl<NumericTypes: EvalexprNumericTypes<Int = Self>> EvalexprInt<NumericTypes> for i64 {
    const MIN: Self = i64::MIN;
    const MAX: Self = i64::MAX;

    fn abs(&self) -> EvalexprResult<Self, NumericTypes> {
        Ok(i64::abs(*self))
    }
}
impl<NumericTypes: EvalexprNumericTypes<Float = Self>> EvalexprFloat<NumericTypes> for f64{
    const MIN: Self = f64::NEG_INFINITY;
    const MAX: Self = f64::INFINITY;

    fn abs(&self) -> Self {
        f64::abs(*self)
    }

    fn random() -> EvalexprResult<Self, NumericTypes> {
        cfg_if! {
            if #[cfg(feature = "rand")] {
                Ok(rand::random())
            } else {
                Err(EvalexprError::RandNotEnabled)
            }
        }
    }
}
