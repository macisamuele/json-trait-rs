use crate::{
    json_type::{JsonMap, JsonMapTrait, JsonType, JsonTypeToString, ThreadSafeJsonType, ToRustType},
    rust_type_impl::RustType,
};
use serde_json::Value;

impl Into<RustType> for Value {
    fn into(self) -> RustType {
        self.to_rust_type()
    }
}

impl ToRustType for Value {}

impl JsonTypeToString for Value {
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}

impl<'json> JsonMapTrait<'json, Value> for JsonMap<'json, Value> {
    #[must_use]
    fn keys(&'json self) -> Box<dyn Iterator<Item = &str> + 'json> {
        #[allow(clippy::option_if_let_else)]
        if let Some(obj) = self.as_object() {
            Box::new(obj.keys().map(AsRef::as_ref))
        } else {
            #[allow(unsafe_code)]
            unsafe {
                std::hint::unreachable_unchecked()
            }
        }
    }

    #[must_use]
    fn values(&'json self) -> Box<dyn Iterator<Item = &Value> + 'json> {
        #[allow(clippy::option_if_let_else)]
        if let Some(obj) = self.as_object() {
            Box::new(obj.values())
        } else {
            #[allow(unsafe_code)]
            unsafe {
                std::hint::unreachable_unchecked()
            }
        }
    }

    #[must_use]
    fn items(&'json self) -> Box<dyn Iterator<Item = (&str, &Value)> + 'json> {
        #[allow(clippy::option_if_let_else)]
        if let Some(obj) = self.as_object() {
            Box::new(obj.iter().map(|(k, v)| (k.as_ref(), v)))
        } else {
            #[allow(unsafe_code)]
            unsafe {
                std::hint::unreachable_unchecked()
            }
        }
    }
}

impl JsonType for Value {
    #[must_use]
    fn as_array<'json>(&'json self) -> Option<Box<dyn ExactSizeIterator<Item = &Self> + 'json>> {
        self.as_array().map(|vec| {
            let b: Box<dyn ExactSizeIterator<Item = _>> = Box::new(vec.iter());
            b
        })
    }

    #[must_use]
    fn as_boolean(&self) -> Option<bool> {
        self.as_bool()
    }

    #[must_use]
    fn as_integer(&self) -> Option<i128> {
        self.as_i64().map(i128::from)
    }

    #[must_use]
    fn as_null(&self) -> Option<()> {
        self.as_null()
    }

    #[must_use]
    fn as_number(&self) -> Option<f64> {
        self.as_f64()
    }

    #[must_use]
    fn as_object(&self) -> Option<JsonMap<Self>> {
        if self.is_object() {
            Some(JsonMap::new(self))
        } else {
            None
        }
    }

    #[must_use]
    fn as_string(&self) -> Option<&str> {
        self.as_str()
    }

    #[must_use]
    fn get_attribute(&self, attribute_name: &str) -> Option<&Self> {
        self.get(attribute_name)
    }

    #[must_use]
    fn get_index(&self, index: usize) -> Option<&Self> {
        self.get(index)
    }

    #[must_use]
    fn has_attribute(&self, attribute_name: &str) -> bool {
        self.get(attribute_name).is_some()
    }
}

impl ThreadSafeJsonType for Value {}

#[cfg(test)]
mod tests_json_map_trait {
    use crate::json_type::{JsonMap, JsonMapTrait};
    use serde_json::Value;

    lazy_static! {
        static ref TESTING_MAP: Value = json![{"k1": "v1", "k2": "v2"}];
    }

    #[test]
    fn keys() {
        let testing_map: &Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).keys().collect::<Vec<_>>(), vec!["k1", "k2"]);
    }

    #[test]
    fn values() {
        let testing_map: &Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).values().collect::<Vec<_>>(), vec![&json!["v1"], &json!["v2"]]);
    }

    #[test]
    fn items() {
        let testing_map: &Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).items().collect::<Vec<_>>(), vec![("k1", &json!["v1"]), ("k2", &json!["v2"])]);
    }
}

#[cfg(test)]
mod tests_primitive_type_trait {
    use crate::json_type::{JsonType, PrimitiveType};
    use serde_json::Value;
    use std::ops::Deref;
    use test_case::test_case;

    #[test_case(&json![[]], PrimitiveType::Array)]
    #[test_case(&json![true], PrimitiveType::Boolean)]
    #[test_case(&json![1], PrimitiveType::Integer)]
    #[test_case(&json![null], PrimitiveType::Null)]
    #[test_case(&json![1.2], PrimitiveType::Number)]
    #[test_case(&json![{"prop": "value"}], PrimitiveType::Object)]
    #[test_case(&json!["string"], PrimitiveType::String)]
    fn test_primitive_type(value: &Value, expected_value: PrimitiveType) {
        assert_eq!(JsonType::primitive_type(value), expected_value);
    }

    #[test_case(&json![{"present": 1}], "present",  Some(&json![1]))]
    #[test_case(&json![{"present": 1}], "not-present", None)]
    fn test_get_attribute(value: &Value, attribute_name: &str, expected_value: Option<&Value>) {
        assert_eq!(JsonType::get_attribute(value, attribute_name), expected_value);
    }

    #[test_case(&json![[0, 1, 2]], 1, &Some(json![1]))]
    #[test_case(&json![[0, 1, 2]], 4, &None)]
    fn test_get_index(value: &Value, index: usize, expected_value: &Option<Value>) {
        assert_eq!(JsonType::get_index(value, index), expected_value.as_ref());
    }

    #[test_case(&json![{"present": 1}], "present", true)]
    #[test_case(&json![{"present": 1}], "not-present", false)]
    #[test_case(&json![[1, 2, 3]], "not-present", false)]
    fn test_has_attribute(value: &Value, attr_name: &str, expected_value: bool) {
        assert_eq!(JsonType::has_attribute(value, attr_name), expected_value);
    }

    #[test_case(&json![[0, 1, 2]], true)]
    #[test_case(&json![true], false)]
    #[test_case(&json![1_u32], false)]
    #[test_case(&json![null], false)]
    #[test_case(&json![1.2_f32], false)]
    #[test_case(&json![{"key": "value"}], false)]
    #[test_case(&json!["string"], false)]
    fn test_is_array(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_array(value), expected_value);
    }

    #[test_case(&json![[0, 1, 2]], false)]
    #[test_case(&json![true], true)]
    #[test_case(&json![1_u32], false)]
    #[test_case(&json![null], false)]
    #[test_case(&json![1.2_f32], false)]
    #[test_case(&json![{"key": "value"}], false)]
    #[test_case(&json!["string"], false)]
    fn test_is_boolean(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_boolean(value), expected_value);
    }

    #[test_case(&json![[0, 1, 2]], false)]
    #[test_case(&json![true], false)]
    #[test_case(&json![1_u32], true)]
    #[test_case(&json![null], false)]
    #[test_case(&json![1.2_f32], false)]
    #[test_case(&json![{"key": "value"}], false)]
    #[test_case(&json!["string"], false)]
    fn test_is_integer(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_integer(value), expected_value);
    }

    #[test_case(&json![[0, 1, 2]], false)]
    #[test_case(&json![true], false)]
    #[test_case(&json![1_u32], false)]
    #[test_case(&json![null], true)]
    #[test_case(&json![1.2_f32], false)]
    #[test_case(&json![{"key": "value"}], false)]
    #[test_case(&json!["string"], false)]
    fn test_is_null(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_null(value), expected_value);
    }

    #[test_case(&json![[0, 1, 2]], false)]
    #[test_case(&json![true], false)]
    #[test_case(&json![1_u32], true)]
    #[test_case(&json![null], false)]
    #[test_case(&json![1.2_f32], true)]
    #[test_case(&json![{"key": "value"}], false)]
    #[test_case(&json!["string"], false)]
    fn test_is_number(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_number(value), expected_value);
    }

    #[test_case(&json![[0, 1, 2]], false)]
    #[test_case(&json![true], false)]
    #[test_case(&json![1_u32], false)]
    #[test_case(&json![null], false)]
    #[test_case(&json![1.2_f32], false)]
    #[test_case(&json![{"key": "value"}], true)]
    #[test_case(&json!["string"], false)]
    fn test_is_object(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_object(value), expected_value);
    }

    #[test_case(&json![[0, 1, 2]], false)]
    #[test_case(&json![true], false)]
    #[test_case(&json![1_u32], false)]
    #[test_case(&json![null], false)]
    #[test_case(&json![1.2_f32], false)]
    #[test_case(&json![{"key": "value"}], false)]
    #[test_case(&json!["string"], true)]
    fn test_is_string(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_string(value), expected_value);
    }

    #[test_case(&json![[1]], &Some(vec![json![1]]))]
    #[test_case(&json![[1, "a"]], &Some(vec![json![1], json!["a"]]))]
    #[test_case(&json![null], &None)]
    fn test_as_array(value: &Value, expected_value: &Option<Vec<Value>>) {
        assert_eq!(&JsonType::as_array(value).map(|iterator| iterator.cloned().collect()), expected_value);
    }

    #[test_case(&json![true], Some(true))]
    #[test_case(&json![false], Some(false))]
    #[test_case(&json![1], None)]
    fn test_as_boolean(value: &Value, expected_value: Option<bool>) {
        assert_eq!(JsonType::as_boolean(value), expected_value);
    }

    #[test_case(&json![1], Some(1))]
    #[test_case(&json![1.2], None)]
    #[test_case(&json!["1"], None)]
    fn test_as_integer(value: &Value, expected_value: Option<i128>) {
        assert_eq!(JsonType::as_integer(value), expected_value);
    }

    #[test_case(&json![null], Some(()))]
    #[test_case(&json!["1"], None)]
    fn test_as_null(value: &Value, expected_value: Option<()>) {
        assert_eq!(JsonType::as_null(value), expected_value);
    }

    #[test_case(&json![1], Some(1_f64))]
    #[test_case(&json![1.2], Some(1.2))]
    #[test_case(&json!["1"], None)]
    fn test_as_number(value: &Value, expected_value: Option<f64>) {
        assert_eq!(JsonType::as_number(value), expected_value);
    }

    #[test_case(&json![1], &None)]
    #[test_case(&json![1.2], &None)]
    #[test_case(&json![{"1": 1}], &Some(json![{"1": 1}]))]
    fn test_as_object(value: &Value, expected_value: &Option<Value>) {
        assert_eq!(
            match JsonType::as_object(value) {
                Some(ref v) => Some({
                    #[allow(clippy::explicit_deref_methods)] // Explicit deref call is needed to ensure that &Value is retrieved from JsonMap
                    v.deref()
                }),
                None => None,
            },
            expected_value.as_ref(),
        );
    }

    #[test_case(&json![1], None)]
    #[test_case(&json![1.2], None)]
    #[test_case(&json!["1"], Some("1"))]
    fn test_as_string(value: &Value, expected_value: Option<&str>) {
        assert_eq!(JsonType::as_string(value), expected_value);
    }
}

#[cfg(test)]
mod tests_json_map {
    use crate::json_type::{JsonMapTrait, JsonType};
    use serde_json::Value;

    lazy_static! {
        static ref TESTING_MAP: Value = json![{"key1": {"key2": 1}}];
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
            vec![format!("{:?}", Value::from(1))],
        );
    }

    #[test]
    fn test_items() {
        let key1 = TESTING_MAP.get_attribute("key1").unwrap();
        assert_eq!(
            JsonType::as_object(key1).unwrap().items().map(|(k, v)| format!("{} -> {:?}", k, v)).collect::<Vec<_>>(),
            vec![format!("key2 -> {:?}", Value::from(1))],
        );
    }
}

#[cfg(test)]
mod tests_to_json_string {
    use crate::json_type::JsonTypeToString;

    #[test]
    fn smoke_test() {
        let value = json![[
            {"array": []},
            {"boolean": false},
            {"float": 2.3},
            {"integer": 1},
            {"null": null},
            {"object": {}},
            {"string": "string"},
        ]];
        assert_eq!(
            value.to_json_string(),
            r#"[{"array":[]},{"boolean":false},{"float":2.3},{"integer":1},{"null":null},{"object":{}},{"string":"string"}]"#
        );
    }
}
