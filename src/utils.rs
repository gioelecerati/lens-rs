use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn get_field_by_name<T, R>(data: T, field: &str) -> R
where
    T: Serialize,
    R: DeserializeOwned,
{
    let mut map = match serde_value::to_value(data) {
        Ok(serde_value::Value::Map(map)) => map,
        _ => panic!("expected a struct"),
    };

    let key = serde_value::Value::String(field.to_owned());
    let value = match map.remove(&key) {
        Some(value) => value,
        None => panic!("no such field"),
    };

    match R::deserialize(value) {
        Ok(r) => r,
        Err(_) => panic!("wrong type?"),
    }
}
