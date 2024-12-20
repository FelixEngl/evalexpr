#![cfg(not(tarpaulin_include))]
#![cfg(feature = "num")]

use evalexpr::{ContextWithMutableVariables, ConvertibleContext, DefaultNumericTypes, HashMapContext, Value};

#[test]
fn test_convert() {
    let mut m: HashMapContext = HashMapContext::default();
    m.set_value("a".to_string(), Value::Float(1.0)).unwrap();
    m.try_convert_to::<HashMapContext<DefaultNumericTypes>>().expect("The conversion failed!");
}