use crate::json_type::{JsonMap, JsonMapTrait, JsonType};
use std::{collections::hash_map::HashMap, ops::Deref};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RustType {
    Null,
    Boolean(bool),
    String(String),
    Integer(i32),
    List(Vec<RustType>),
    Object(HashMap<String, RustType>),
}

impl Default for RustType {
    fn default() -> Self {
        Self::Null
    }
}

impl From<()> for RustType {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<bool> for RustType {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<&str> for RustType {
    fn from(value: &str) -> Self {
        Self::String(String::from(value))
    }
}

impl From<String> for RustType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i32> for RustType {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<HashMap<String, RustType>> for RustType {
    fn from(value: HashMap<String, Self>) -> Self {
        Self::Object(value)
    }
}

impl From<Vec<RustType>> for RustType {
    fn from(value: Vec<Self>) -> Self {
        Self::List(value)
    }
}

impl JsonType<RustType> for RustType {
    fn as_array<'json>(&'json self) -> Option<Box<dyn Iterator<Item = &Self> + 'json>> {
        if let Self::List(v) = self {
            Some(Box::new(v.iter()))
        } else {
            None
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        if let Self::Boolean(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    fn as_integer(&self) -> Option<i128> {
        if let Self::Integer(v) = self {
            Some(i128::from(*v))
        } else {
            None
        }
    }

    fn as_null(&self) -> Option<()> {
        if let Self::Null = self {
            Some(())
        } else {
            None
        }
    }

    fn as_number(&self) -> Option<f64> {
        if let Self::Integer(v) = self {
            Some(f64::from(*v))
        } else {
            None
        }
    }

    fn as_object(&self) -> Option<JsonMap<Self>>
    where
        for<'json> JsonMap<'json, Self>: JsonMapTrait<'json, Self>,
    {
        if let Self::Object(_) = self {
            Some(JsonMap::new(self))
        } else {
            None
        }
    }

    fn as_string(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    fn get_attribute(&self, attribute_name: &str) -> Option<&Self> {
        if let Self::Object(object) = self {
            object.get(attribute_name)
        } else {
            None
        }
    }

    fn get_index(&self, index: usize) -> Option<&Self> {
        if let Self::List(array) = self {
            array.get(index)
        } else {
            None
        }
    }
}

impl<'json> JsonMapTrait<'json, RustType> for JsonMap<'json, RustType> {
    #[inline]
    fn items(&'json self) -> Box<dyn Iterator<Item = (&str, &RustType)> + 'json> {
        if let RustType::Object(hash_map) = self.deref() {
            Box::new(hash_map.iter().map(|(k, v)| (k.as_str(), v)))
        } else {
            #[allow(unsafe_code)]
            unsafe {
                unreachable::unreachable()
            }
        }
    }
}

#[cfg(test)]
mod smoke_test {
    use crate::{
        json_type::{JsonMapTrait, JsonType},
        rust_type::RustType,
    };
    use std::collections::hash_map::HashMap;

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
        assert_eq!(testing_type_instance.is_number(), true);
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
    use crate::{
        json_type::{JsonMapTrait, JsonType},
        RustType,
    };

    lazy_static! {
        static ref TESTING_MAP: RustType = rust_type_map!(
            "key1" => rust_type_map!(
                "key2" => 1,
            ),
        );
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
