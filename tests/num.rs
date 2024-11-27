#![cfg(not(tarpaulin_include))]
#![cfg(feature = "num")]

use std::collections::HashMap;
use evalexpr::{context_map, ContextWithMutableVariables, ConvertibleContext, DefaultNumericTypes, EmptyContext, HashMapContext, Value};

#[test]
fn test_convert() {
    let mut m: HashMapContext = HashMapContext::default();
    m.set_value("a".to_string(), Value::Float(1.0)).unwrap();
    let x = m.try_convert_to::<HashMapContext<DefaultNumericTypes>>();

    let mut m: HashMapContext = context_map! {
        "hello" => conv 1
    }.unwrap();
}