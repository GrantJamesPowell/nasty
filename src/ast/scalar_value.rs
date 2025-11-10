use std::{collections::HashMap, sync::Arc};

pub enum ScalarValue {
    Bool(bool),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Text(Arc<str>),
    Bytea(Arc<[u8]>),
    Array(Arc<[ScalarValue]>),
    Map(HashMap<String, ScalarValue>),
}

impl From<bool> for ScalarValue {
    fn from(value: bool) -> Self {
        ScalarValue::Bool(value)
    }
}

impl From<i32> for ScalarValue {
    fn from(value: i32) -> Self {
        ScalarValue::Int32(value)
    }
}

impl From<i64> for ScalarValue {
    fn from(value: i64) -> Self {
        ScalarValue::Int64(value)
    }
}

impl From<f32> for ScalarValue {
    fn from(value: f32) -> Self {
        ScalarValue::Float32(value)
    }
}

impl From<f64> for ScalarValue {
    fn from(value: f64) -> Self {
        ScalarValue::Float64(value)
    }
}

impl From<String> for ScalarValue {
    fn from(value: String) -> Self {
        ScalarValue::Text(Arc::from(value.into_boxed_str()))
    }
}

impl<K, V> From<HashMap<K, V>> for ScalarValue
where
    K: Into<String>,
    V: Into<ScalarValue>,
{
    fn from(map: HashMap<K, V>) -> Self {
        ScalarValue::Map(map.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
    }
}
