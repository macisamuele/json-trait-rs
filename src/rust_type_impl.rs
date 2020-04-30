use crate::{
    json_type::{JsonMap, JsonMapTrait, JsonType, ToRustType},
    ThreadSafeJsonType,
};
use join_lazy_fmt::Join;
use std::{collections::hash_map::HashMap, fmt, ops::Deref};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, PartialEq)]
pub enum RustType {
    Null,
    Boolean(bool),
    String(String),
    Integer(i128),
    Number(f64),
    List(Vec<RustType>),
    Object(HashMap<String, RustType>),
}

impl fmt::Display for RustType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Null => write!(formatter, "null"),
            Self::Boolean(value) => write!(formatter, "{}", value),
            Self::String(value) => write!(formatter, r#""{}""#, value),
            Self::Integer(value) => write!(formatter, "{}", value),
            Self::Number(value) => write!(formatter, "{}", value),
            Self::List(value) => {
                write!(formatter, "[")?;
                write!(formatter, "{}", ",".join(value))?;
                write!(formatter, "]")
            }
            Self::Object(value) => {
                write!(formatter, "{{")?;
                write!(formatter, "{}", ",".join(value.iter().map(|(key, value)| { format!(r#""{}":{}"#, key, value) })))?;
                write!(formatter, "}}")
            }
        }
    }
}

impl Default for RustType {
    #[must_use]
    fn default() -> Self {
        Self::Null
    }
}

impl From<()> for RustType {
    #[must_use]
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<bool> for RustType {
    #[must_use]
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<&str> for RustType {
    #[must_use]
    fn from(value: &str) -> Self {
        Self::String(String::from(value))
    }
}

impl From<String> for RustType {
    #[must_use]
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i32> for RustType {
    #[must_use]
    fn from(value: i32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i64> for RustType {
    #[must_use]
    fn from(value: i64) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i128> for RustType {
    #[must_use]
    fn from(value: i128) -> Self {
        Self::Integer(value)
    }
}

impl From<f32> for RustType {
    #[must_use]
    fn from(value: f32) -> Self {
        Self::Number(value.into())
    }
}

impl From<f64> for RustType {
    #[must_use]
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<HashMap<String, RustType>> for RustType {
    #[must_use]
    fn from(value: HashMap<String, Self>) -> Self {
        Self::Object(value)
    }
}

impl From<Vec<RustType>> for RustType {
    #[must_use]
    fn from(value: Vec<Self>) -> Self {
        Self::List(value)
    }
}

impl ToRustType for RustType {
    fn to_rust_type(&self) -> RustType {
        self.clone()
    }
}

impl JsonType for RustType {
    #[must_use]
    fn as_array<'json>(&'json self) -> Option<Box<dyn ExactSizeIterator<Item = &Self> + 'json>> {
        if let Self::List(v) = self {
            Some(Box::new(v.iter()))
        } else {
            None
        }
    }

    #[must_use]
    fn as_boolean(&self) -> Option<bool> {
        if let Self::Boolean(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    #[must_use]
    fn as_integer(&self) -> Option<i128> {
        if let Self::Integer(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    #[must_use]
    fn as_null(&self) -> Option<()> {
        if let Self::Null = self {
            Some(())
        } else {
            None
        }
    }

    #[must_use]
    fn as_number(&self) -> Option<f64> {
        if let Self::Number(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    #[must_use]
    fn as_object(&self) -> Option<JsonMap<Self>> {
        if let Self::Object(_) = self {
            Some(JsonMap::new(self))
        } else {
            None
        }
    }

    #[must_use]
    fn as_string(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    #[must_use]
    fn get_attribute(&self, attribute_name: &str) -> Option<&Self> {
        if let Self::Object(object) = self {
            object.get(attribute_name)
        } else {
            None
        }
    }

    #[must_use]
    fn get_index(&self, index: usize) -> Option<&Self> {
        if let Self::List(array) = self {
            array.get(index)
        } else {
            None
        }
    }
}

impl ThreadSafeJsonType for RustType {}

impl<'json> JsonMapTrait<'json, RustType> for JsonMap<'json, RustType> {
    #[must_use]
    fn items(&'json self) -> Box<dyn Iterator<Item = (&str, &RustType)> + 'json> {
        if let RustType::Object(hash_map) = {
            #[allow(clippy::explicit_deref_methods)] // Explicit deref call is needed to ensure that &RustType is retrieved from JsonMap
            &self.deref()
        } {
            Box::new(hash_map.iter().map(|(k, v)| (k.as_str(), v)))
        } else {
            #[allow(unsafe_code)]
            unsafe {
                std::hint::unreachable_unchecked()
            }
        }
    }
}

#[cfg(test)]
mod smoke_test {
    use crate::{
        json_type::{JsonMapTrait, JsonType},
        rust_type_impl::RustType,
    };
    use std::collections::hash_map::HashMap;
    use test_case::test_case;

    #[test_case(&rust_type!(()) => "null")]
    #[test_case(&rust_type!(true) => "true")]
    #[test_case(&rust_type!(false) => "false")]
    #[test_case(&rust_type!(1) => "1")]
    #[test_case(&rust_type!(2.3) => "2.3")]
    #[test_case(&rust_type!([1, 2.3, false]) => "[1,2.3,false]")]
    fn test_to_string(value: &RustType) -> String {
        value.to_string()
    }

    #[test]
    fn test_testing_type_instance_string() {
        let string = "string";
        let testing_type_instance = RustType::from(string);
        assert_eq!(testing_type_instance.as_string(), Some(string));
        assert_eq!(testing_type_instance.has_attribute("attribute"), false);
        assert_eq!(testing_type_instance.is_array(), false);
        assert_eq!(testing_type_instance.is_boolean(), false);
        assert_eq!(testing_type_instance.is_integer(), false);
        assert_eq!(testing_type_instance.is_null(), false);
        assert_eq!(testing_type_instance.is_number(), false);
        assert_eq!(testing_type_instance.is_object(), false);
        assert_eq!(testing_type_instance.is_string(), true);
    }

    #[test]
    fn test_testing_type_instance_integer() {
        let integer = 1;
        let testing_type_instance = RustType::from(integer);
        assert_eq!(testing_type_instance.as_integer(), Some(i128::from(integer)));
        assert_eq!(testing_type_instance.has_attribute("attribute"), false);
        assert_eq!(testing_type_instance.is_array(), false);
        assert_eq!(testing_type_instance.is_boolean(), false);
        assert_eq!(testing_type_instance.is_integer(), true);
        assert_eq!(testing_type_instance.is_null(), false);
        assert_eq!(testing_type_instance.is_number(), false);
        assert_eq!(testing_type_instance.is_object(), false);
        assert_eq!(testing_type_instance.is_string(), false);
    }

    #[test]
    fn test_testing_type_instance_list() {
        let array = vec![RustType::from(1), RustType::from(2)];
        let testing_type_instance = RustType::from(array.clone());
        assert_eq!(testing_type_instance.as_array().map(Iterator::collect::<Vec<_>>), Some(array.iter().collect()));
        assert_eq!(testing_type_instance.has_attribute("attribute"), false);
        assert_eq!(testing_type_instance.is_array(), true);
        assert_eq!(testing_type_instance.is_boolean(), false);
        assert_eq!(testing_type_instance.is_integer(), false);
        assert_eq!(testing_type_instance.is_null(), false);
        assert_eq!(testing_type_instance.is_number(), false);
        assert_eq!(testing_type_instance.is_object(), false);
        assert_eq!(testing_type_instance.is_string(), false);
    }

    #[test]
    fn test_testing_type_instance_object() {
        let object: HashMap<String, RustType> = [("attribute".to_string(), RustType::from("value"))].iter().cloned().collect();
        let testing_type_instance = RustType::from(object);
        assert_eq!(
            testing_type_instance.as_object().unwrap().items().collect::<Vec<_>>(),
            vec![("attribute", &RustType::from("value"))],
        );
        assert_eq!(testing_type_instance.has_attribute("attribute"), true);
        assert_eq!(testing_type_instance.is_array(), false);
        assert_eq!(testing_type_instance.is_boolean(), false);
        assert_eq!(testing_type_instance.is_integer(), false);
        assert_eq!(testing_type_instance.is_null(), false);
        assert_eq!(testing_type_instance.is_number(), false);
        assert_eq!(testing_type_instance.is_object(), true);
        assert_eq!(testing_type_instance.is_string(), false);
    }
}

#[cfg(test)]
mod json_map_tests {
    use super::RustType;
    use crate::json_type::{JsonMapTrait, JsonType};

    lazy_static! {
        static ref TESTING_MAP: RustType = rust_type!({"key1": {"key2": 1}});
    }

    #[test]
    fn test_keys() {
        let key1 = TESTING_MAP.get_attribute("key1").unwrap();
        assert_eq!(JsonType::as_object(key1).unwrap().keys().collect::<Vec<_>>(), vec![String::from("key2")]);
    }

    #[test]
    fn test_values() {
        let key1 = TESTING_MAP.get_attribute("key1").unwrap();
        assert_eq!(
            JsonType::as_object(key1).unwrap().values().map(|v| format!("{:?}", v)).collect::<Vec<_>>(),
            vec![format!("{:?}", RustType::from(1))],
        );
    }

    #[test]
    fn test_items() {
        let key1 = TESTING_MAP.get_attribute("key1").unwrap();
        assert_eq!(
            JsonType::as_object(key1).unwrap().items().map(|(k, v)| format!("{} -> {:?}", k, v)).collect::<Vec<_>>(),
            vec![format!("key2 -> {:?}", RustType::from(1))],
        );
    }
}
