use crate::error::{EvalexprError, EvalexprResult, EvalexprResultValue};
use std::{convert::TryFrom, ops::RangeInclusive};
use self::numeric_types::{DefaultNumericTypes, EvalexprNumericTypes};


#[cfg(feature = "num")]
use numeric_types::with_num::EvalexprNumericTypesConvert;

mod display;
pub mod numeric_types;
pub mod value_type;

/// The type used to represent tuples in `Value::Tuple`.
pub type TupleType<NumericTypes = DefaultNumericTypes> = Vec<Value<NumericTypes>>;

/// The type used to represent empty values in `Value::Empty`.
pub type EmptyType = ();

/// The value of the empty type to be used in rust.
pub const EMPTY_VALUE: () = ();

/// The value type used by the parser.
/// Values can be of different subtypes that are the variants of this enum.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value<NumericTypes: EvalexprNumericTypes = DefaultNumericTypes> {
    /// A string value.
    String(String),
    /// A float value.
    Float(NumericTypes::Float),
    /// An integer value.
    Int(NumericTypes::Int),
    /// A boolean value.
    Boolean(bool),
    /// A tuple value.
    Tuple(TupleType<NumericTypes>),
    /// An empty value.
    Empty,
}

impl<NumericTypes: EvalexprNumericTypes> Value<NumericTypes> {
    /// Returns true if `self` is a `Value::String`.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    /// Returns true if `self` is a `Value::Int`.
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// Returns true if `self` is a `Value::Float`.
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Int` or `Value::Float`.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Boolean`.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Returns true if `self` is a `Value::Tuple`.
    pub fn is_tuple(&self) -> bool {
        matches!(self, Value::Tuple(_))
    }

    /// Returns true if `self` is a `Value::Empty`.
    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty)
    }

    /// Clones the value stored in `self` as `String`, or returns `Err` if `self` is not a `Value::String`.
    pub fn as_string(&self) -> EvalexprResult<String, NumericTypes> {
        match self {
            Value::String(string) => Ok(string.clone()),
            value => Err(EvalexprError::expected_string(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `IntType`, or returns `Err` if `self` is not a `Value::Int`.
    pub fn as_int(&self) -> EvalexprResult<NumericTypes::Int, NumericTypes> {
        match self {
            Value::Int(i) => Ok(i.clone()),
            value => Err(EvalexprError::expected_int(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float`.
    pub fn as_float(&self) -> EvalexprResult<NumericTypes::Float, NumericTypes> {
        match self {
            Value::Float(f) => Ok(f.clone()),
            value => Err(EvalexprError::expected_float(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float` or `Value::Int`.
    /// Note that this method silently converts `IntType` to `FloatType`, if `self` is a `Value::Int`.
    pub fn as_number(&self) -> EvalexprResult<NumericTypes::Float, NumericTypes> {
        match self {
            Value::Float(f) => Ok(f.clone()),
            Value::Int(i) => Ok(NumericTypes::int_as_float(i)),
            value => Err(EvalexprError::expected_number(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `bool`, or returns `Err` if `self` is not a `Value::Boolean`.
    pub fn as_boolean(&self) -> EvalexprResult<bool, NumericTypes> {
        match self {
            Value::Boolean(boolean) => Ok(*boolean),
            value => Err(EvalexprError::expected_boolean(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType`, or returns `Err` if `self` is not a `Value::Tuple`.
    pub fn as_tuple(&self) -> EvalexprResult<TupleType<NumericTypes>, NumericTypes> {
        match self {
            Value::Tuple(tuple) => Ok(tuple.clone()),
            value => Err(EvalexprError::expected_tuple(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType` or returns `Err` if `self` is not a `Value::Tuple` of the required length.
    pub fn as_fixed_len_tuple(
        &self,
        len: usize,
    ) -> EvalexprResult<TupleType<NumericTypes>, NumericTypes> {
        match self {
            Value::Tuple(tuple) => {
                if tuple.len() == len {
                    Ok(tuple.clone())
                } else {
                    Err(EvalexprError::expected_fixed_len_tuple(len, self.clone()))
                }
            },
            value => Err(EvalexprError::expected_tuple(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType` or returns `Err` if `self` is not a `Value::Tuple` with length in the required range.
    pub fn as_ranged_len_tuple(
        &self,
        range: RangeInclusive<usize>,
    ) -> EvalexprResult<TupleType<NumericTypes>, NumericTypes> {
        match self {
            Value::Tuple(tuple) => {
                if range.contains(&tuple.len()) {
                    Ok(tuple.clone())
                } else {
                    Err(EvalexprError::expected_ranged_len_tuple(
                        range,
                        self.clone(),
                    ))
                }
            },
            value => Err(EvalexprError::expected_tuple(value.clone())),
        }
    }

    /// Returns `()`, or returns`Err` if `self` is not a `Value::Tuple`.
    pub fn as_empty(&self) -> EvalexprResult<(), NumericTypes> {
        match self {
            Value::Empty => Ok(()),
            value => Err(EvalexprError::expected_empty(value.clone())),
        }
    }

    /// Returns a string for the `str::from` built-in function.
    pub fn str_from(&self) -> String {
        match self {
            Value::String(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Int(v) => v.to_string(),
            Value::Boolean(v) => v.to_string(),
            Value::Tuple(_) => self.to_string(),
            Value::Empty => String::from("()"),
        }
    }

    /// Create a new `Value` from its corresponding raw float type.
    pub fn from_float(float: NumericTypes::Float) -> Self {
        Self::Float(float)
    }

    /// Create a new `Value` from its corresponding raw int type.
    pub fn from_int(int: NumericTypes::Int) -> Self {
        Self::Int(int)
    }

    #[cfg(feature = "num")]
    /// Create a new `Value` from a castable float-like type.
    pub fn from_as_float<T>(float: T) -> Self
    where
        T: num_traits::AsPrimitive<NumericTypes::Float> + num_traits::float::Float,
        NumericTypes::Float: Copy
    {
        Self::Float(float.as_())
    }
    

    #[cfg(feature = "num")]
    /// Create a new `Value` from a castable int-like type.
    pub fn from_as_int<T>(int: T) -> Self
    where
        T: num_traits::AsPrimitive<NumericTypes::Int> + num_traits::int::PrimInt,
        NumericTypes::Int: Copy
    {
        Self::Int(int.as_())
    }

}

#[cfg(feature = "num")]
impl<NumericTypes: EvalexprNumericTypesConvert> Value<NumericTypes> {

    /// Cast a value to a float.
    pub fn from_cast_float<T>(float: T) -> EvalexprResult<Self, NumericTypes>
    where
        T: num_traits::NumCast
    {
        match num_traits::cast(float) {
            Some(v) => Ok(Self::Float(v)),
            None => Err(EvalexprError::FloatCastError),
        }
    }

    /// Cast a value to a int
    pub fn from_cast_int<T>(int: T) -> EvalexprResult<Self, NumericTypes>
    where
        T: num_traits::NumCast
    {
        match num_traits::cast(int) {
            Some(v) => Ok(Self::Int(v)),
            None => Err(EvalexprError::IntCastError),
        }
    }
} 

impl<NumericTypes: EvalexprNumericTypes> From<String> for Value<NumericTypes> {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl<NumericTypes: EvalexprNumericTypes> From<&str> for Value<NumericTypes> {
    fn from(string: &str) -> Self {
        Value::String(string.to_string())
    }
}

impl<NumericTypes: EvalexprNumericTypes> From<bool> for Value<NumericTypes> {
    fn from(boolean: bool) -> Self {
        Value::Boolean(boolean)
    }
}

impl<NumericTypes: EvalexprNumericTypes> From<TupleType<NumericTypes>> for Value<NumericTypes> {
    fn from(tuple: TupleType<NumericTypes>) -> Self {
        Value::Tuple(tuple)
    }
}

impl<NumericTypes: EvalexprNumericTypes> From<Value<NumericTypes>>
    for EvalexprResultValue<NumericTypes>
{
    fn from(value: Value<NumericTypes>) -> Self {
        Ok(value)
    }
}

impl<NumericTypes: EvalexprNumericTypes> From<()> for Value<NumericTypes> {
    fn from(_: ()) -> Self {
        Value::Empty
    }
}

impl<NumericTypes: EvalexprNumericTypes> TryFrom<Value<NumericTypes>> for String {
    type Error = EvalexprError<NumericTypes>;

    fn try_from(value: Value<NumericTypes>) -> Result<Self, Self::Error> {
        if let Value::String(value) = value {
            Ok(value)
        } else {
            Err(EvalexprError::ExpectedString { actual: value })
        }
    }
}

impl<NumericTypes: EvalexprNumericTypes> TryFrom<Value<NumericTypes>> for bool {
    type Error = EvalexprError<NumericTypes>;

    fn try_from(value: Value<NumericTypes>) -> Result<Self, Self::Error> {
        if let Value::Boolean(value) = value {
            Ok(value)
        } else {
            Err(EvalexprError::ExpectedBoolean { actual: value })
        }
    }
}

impl<NumericTypes: EvalexprNumericTypes> TryFrom<Value<NumericTypes>> for TupleType<NumericTypes> {
    type Error = EvalexprError<NumericTypes>;

    fn try_from(value: Value<NumericTypes>) -> Result<Self, Self::Error> {
        if let Value::Tuple(value) = value {
            Ok(value)
        } else {
            Err(EvalexprError::ExpectedTuple { actual: value })
        }
    }
}

impl<NumericTypes: EvalexprNumericTypes> TryFrom<Value<NumericTypes>> for () {
    type Error = EvalexprError<NumericTypes>;

    fn try_from(value: Value<NumericTypes>) -> Result<Self, Self::Error> {
        if let Value::Empty = value {
            Ok(())
        } else {
            Err(EvalexprError::ExpectedEmpty { actual: value })
        }
    }
}

#[cfg(feature = "num")]
impl<NumericTypesA> Value<NumericTypesA> where NumericTypesA: EvalexprNumericTypesConvert {
    /// Try to convert a numeric value type
    pub fn try_as_<NumericTypesB>(&self) -> Result<Value<NumericTypesB>, EvalexprError<NumericTypesA>>
    where
        NumericTypesB: EvalexprNumericTypesConvert
    {
        try_convert(self)
    }
}


#[cfg(feature = "num")]
/// Converts a `value` to another type
pub fn try_convert<NumericTypesA, NumericTypesB>(value: &Value<NumericTypesA>) -> Result<Value<NumericTypesB>, EvalexprError<NumericTypesA>>
where
    NumericTypesA: EvalexprNumericTypesConvert,
    NumericTypesB: EvalexprNumericTypesConvert,
{
    use num_traits::cast;
    use std::any::type_name;

    match value {
        Value::String(value) => Ok(Value::String(value.clone())),
        Value::Float(value) => {
            match cast(value.clone()) {
                Some(value) => Ok(Value::Float(value)),
                None => Err(EvalexprError::FloatToNum {
                    float: value.clone(),
                    target_type: type_name::<NumericTypesB::Float>()
                }),
            }
        }
        Value::Int(value) => {
            match cast(value.clone()) {
                Some(value) => Ok(Value::Int(value)),
                None => Err(EvalexprError::IntToNum {
                    int: value.clone(),
                    target_type: type_name::<NumericTypesB::Float>()
                }),
            }
        }
        Value::Boolean(value) => Ok(Value::Boolean(*value)),
        Value::Tuple(value) => Ok(Value::Tuple(value.iter().map(try_convert).collect::<Result<Vec<_>, _>>()?)),
        Value::Empty => Ok(Value::Empty),
    }
}

// #[cfg(feature = "num")]
// /// Converts a `value` to another type
// pub fn cast<NumericTypesA, NumericTypesB>(value: Value<NumericTypesA>) -> Value<NumericTypesB>
// where
//     NumericTypesA: EvalexprNumericTypesCast<NumericTypesB>,
//     NumericTypesB: EvalexprNumericTypesWithCopy,
// {
//     match value {
//         Value::String(value) => Value::String(value.clone()),
//         Value::Float(value) => Value::Float(value.as_()),
//         Value::Int(value) => Value::Int(value.as_()),
//         Value::Boolean(value) => Value::Boolean(value),
//         Value::Tuple(value) => Value::Tuple(value.into_iter().map(cast).collect::<Vec<_>>()),
//         Value::Empty => Value::Empty,
//     }
// }


#[cfg(test)]
mod tests {
    use crate::value::{numeric_types::DefaultNumericTypes, TupleType, Value};

    #[test]
    fn test_value_conversions() {
        assert_eq!(
            Value::<DefaultNumericTypes>::from("string").as_string(),
            Ok(String::from("string"))
        );
        assert_eq!(Value::<DefaultNumericTypes>::from_int(3).as_int(), Ok(3));
        assert_eq!(
            Value::<DefaultNumericTypes>::from_float(3.3).as_float(),
            Ok(3.3)
        );
        assert_eq!(
            Value::<DefaultNumericTypes>::from(true).as_boolean(),
            Ok(true)
        );
        assert_eq!(
            Value::<DefaultNumericTypes>::from(TupleType::new()).as_tuple(),
            Ok(TupleType::new())
        );
    }

    #[test]
    fn test_value_checks() {
        assert!(Value::<DefaultNumericTypes>::from("string").is_string());
        assert!(Value::<DefaultNumericTypes>::from_int(3).is_int());
        assert!(Value::<DefaultNumericTypes>::from_float(3.3).is_float());
        assert!(Value::<DefaultNumericTypes>::from(true).is_boolean());
        assert!(Value::<DefaultNumericTypes>::from(TupleType::new()).is_tuple());
    }

    #[test]
    fn test_value_str_from() {
        assert_eq!(
            Value::<DefaultNumericTypes>::from("string").str_from(),
            "string"
        );
        assert_eq!(
            Value::<DefaultNumericTypes>::from_float(3.3).str_from(),
            "3.3"
        );
        assert_eq!(Value::<DefaultNumericTypes>::from_int(3).str_from(), "3");
        assert_eq!(Value::<DefaultNumericTypes>::from(true).str_from(), "true");
        assert_eq!(Value::<DefaultNumericTypes>::from(()).str_from(), "()");
        assert_eq!(
            Value::<DefaultNumericTypes>::from(TupleType::from([
                Value::<DefaultNumericTypes>::from("string"),
                Value::<DefaultNumericTypes>::from_float(3.3),
                Value::<DefaultNumericTypes>::from_int(3),
                Value::<DefaultNumericTypes>::from(TupleType::from([
                    Value::<DefaultNumericTypes>::from_int(42),
                    Value::<DefaultNumericTypes>::from_float(4.2),
                ])),
                Value::<DefaultNumericTypes>::from(()),
                Value::<DefaultNumericTypes>::from(true),
            ]))
            .str_from(),
            r#"("string", 3.3, 3, (42, 4.2), (), true)"#
        );
    }
}
