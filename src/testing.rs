use crate::json_type::{JsonMap, JsonMapTrait, JsonType};
use std::{collections::hash_map::HashMap, ops::Deref};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TestingType {
    Null,
    Boolean(bool),
    String(String),
    Integer(i32),
    List(Vec<TestingType>),
    Object(HashMap<String, TestingType>),
}

impl Default for TestingType {
    fn default() -> Self {
        TestingType::Null
    }
}

impl From<()> for TestingType {
    fn from(_: ()) -> Self {
        TestingType::Null
    }
}

impl From<bool> for TestingType {
    fn from(value: bool) -> Self {
        TestingType::Boolean(value)
    }
}

impl From<&str> for TestingType {
    fn from(value: &str) -> Self {
        TestingType::String(String::from(value))
    }
}

impl From<String> for TestingType {
    fn from(value: String) -> Self {
        TestingType::String(value)
    }
}

impl From<i32> for TestingType {
    fn from(value: i32) -> Self {
        TestingType::Integer(value)
    }
}

impl From<HashMap<String, TestingType>> for TestingType {
    fn from(value: HashMap<String, Self>) -> Self {
        TestingType::Object(value)
    }
}

impl From<Vec<TestingType>> for TestingType {
    fn from(value: Vec<Self>) -> Self {
        TestingType::List(value)
    }
}

impl JsonType for TestingType {
    fn as_array<'json>(&'json self) -> Option<Box<ExactSizeIterator<Item = &Self> + 'json>> {
        match self {
            TestingType::List(v) => Some(Box::new(v.iter())),
            _ => None,
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        if let TestingType::Boolean(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    fn as_integer(&self) -> Option<i128> {
        if let TestingType::Integer(v) = self {
            Some(i128::from(*v))
        } else {
            None
        }
    }

    fn as_null(&self) -> Option<()> {
        if let TestingType::Null = self {
            Some(())
        } else {
            None
        }
    }

    fn as_number(&self) -> Option<f64> {
        if let TestingType::Integer(v) = self {
            Some(f64::from(*v))
        } else {
            None
        }
    }

    fn as_object<'json>(&'json self) -> Option<JsonMap<'json, Self>>
    where
        JsonMap<'json, Self>: JsonMapTrait<'json, Self>,
    {
        if let TestingType::Object(_) = self {
            Some(JsonMap::new(self))
        } else {
            None
        }
    }

    fn as_string(&self) -> Option<&str> {
        if let TestingType::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    fn get_attribute<R: AsRef<str>>(&self, attribute_name: R) -> Option<&Self> {
        if let TestingType::Object(object) = self {
            object.get(attribute_name.as_ref())
        } else {
            None
        }
    }

    fn get_index(&self, index: usize) -> Option<&Self> {
        if let TestingType::List(array) = self {
            array.get(index)
        } else {
            None
        }
    }
}

impl<'json> JsonMapTrait<'json, TestingType> for JsonMap<'json, TestingType> {
    #[inline]
    fn items(&'json self) -> Box<ExactSizeIterator<Item = (&str, &TestingType)> + 'json> {
        if let TestingType::Object(hash_map) = self.deref() {
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
        testing::TestingType,
    };
    use std::collections::hash_map::HashMap;

    #[test]
    fn test_testing_type_instance_string() {
        let string = "string";
        let testing_type_instance = TestingType::from(string);
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
        let testing_type_instance = TestingType::from(integer);
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
        let array = vec![TestingType::from(1), TestingType::from(2)];
        let testing_type_instance = TestingType::from(array.clone());
        assert_eq!(
            testing_type_instance.as_array().and_then(|iterator| Some(iterator.collect::<Vec<_>>())),
            Some(array.iter().collect())
        );
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
        let object: HashMap<String, TestingType> = [("attribute".to_string(), TestingType::from("value"))].iter().cloned().collect();
        let testing_type_instance = TestingType::from(object);
        assert_eq!(
            testing_type_instance.as_object().unwrap().items().collect::<Vec<_>>(),
            vec![("attribute", &TestingType::from("value"))],
        );
        assert_eq!(testing_type_instance.get("attribute"), Some(&TestingType::from("value")));
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
