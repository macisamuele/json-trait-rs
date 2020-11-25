use crate::{
    json_type::{JsonMap, JsonMapTrait, JsonType, ThreadSafeJsonType, ToRustType},
    rust_type_impl::RustType,
};
use serde_yaml::Value;

impl Into<RustType> for Value {
    fn into(self) -> RustType {
        self.to_rust_type()
    }
}

impl ToRustType for Value {}

impl<'json> JsonMapTrait<'json, Value> for JsonMap<'json, Value> {
    #[must_use]
    fn keys(&'json self) -> Box<dyn Iterator<Item = &str> + 'json> {
        #[allow(clippy::option_if_let_else)]
        if let Some(obj) = self.as_mapping() {
            Box::new(obj.iter().map(|(key, _)| (key.as_str().unwrap())))
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
        if let Some(obj) = self.as_mapping() {
            Box::new(obj.iter().map(|(_, value)| value))
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
        if let Some(obj) = self.as_mapping() {
            Box::new(obj.iter().map(|(key, value)| (key.as_str().unwrap(), value)))
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
        self.as_sequence().map(|vec| {
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
        if self.as_mapping().is_some() {
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
macro_rules! yaml {
    ($($json:tt)+) => {{
        use serde_json;
        use serde_yaml;
        #[allow(unused_qualifications)]
        let thing: serde_yaml::Value = serde_yaml::from_str(
            serde_json::to_string(&json![$($json)+]).unwrap().as_str(),
        ).unwrap();
        thing
    }};
}

#[cfg(test)]
mod tests_yaml_map_trait {
    use crate::json_type::{JsonMap, JsonMapTrait};
    use serde_yaml::Value;

    lazy_static! {
        static ref TESTING_MAP: Value = yaml![{"k1": "v1", "k2": "v2"}];
    }

    #[test]
    fn keys() {
        let testing_map: &Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).keys().collect::<Vec<_>>(), vec!["k1", "k2"]);
    }

    #[test]
    fn values() {
        let testing_map: &Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).values().collect::<Vec<_>>(), vec![&yaml!["v1"], &yaml!["v2"]]);
    }

    #[test]
    fn items() {
        let testing_map: &Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).items().collect::<Vec<_>>(), vec![("k1", &yaml!["v1"]), ("k2", &yaml!["v2"])]);
    }
}

#[cfg(test)]
mod tests_primitive_type_trait {
    use crate::json_type::{JsonType, PrimitiveType};
    use serde_yaml::Value;
    use std::ops::Deref;
    use test_case::test_case;

    #[test_case(&yaml![[]], PrimitiveType::Array)]
    #[test_case(&yaml![true], PrimitiveType::Boolean)]
    #[test_case(&yaml![1], PrimitiveType::Integer)]
    #[test_case(&yaml![null], PrimitiveType::Null)]
    #[test_case(&yaml![1.2], PrimitiveType::Number)]
    #[test_case(&yaml![{"prop": "value"}], PrimitiveType::Object)]
    #[test_case(&yaml!["string"], PrimitiveType::String)]
    fn test_primitive_type(value: &Value, expected_value: PrimitiveType) {
        assert_eq!(JsonType::primitive_type(value), expected_value);
    }

    #[test_case(&yaml![{"present": 1}], "present", Some(&yaml![1]))]
    #[test_case(&yaml![{"present": 1}], "not-present", None)]
    fn test_get_attribute(value: &Value, attribute_name: &str, expected_value: Option<&Value>) {
        assert_eq!(JsonType::get_attribute(value, attribute_name), expected_value);
    }

    #[test_case(&yaml![[0, 1, 2]], 1, &Some(yaml![1]))]
    #[test_case(&yaml![[0, 1, 2]], 4, &None)]
    fn test_get_index(value: &Value, index: usize, expected_value: &Option<Value>) {
        assert_eq!(JsonType::get_index(value, index), expected_value.as_ref());
    }

    #[test_case(&yaml![{"present": 1}], "present", true)]
    #[test_case(&yaml![{"present": 1}], "not-present", false)]
    #[test_case(&yaml![[1, 2, 3]], "not-present", false)]
    fn test_has_attribute(value: &Value, attr_name: &str, expected_value: bool) {
        assert_eq!(JsonType::has_attribute(value, attr_name), expected_value);
    }

    #[test_case(&yaml![[0, 1, 2]], true)]
    #[test_case(&yaml![true], false)]
    #[test_case(&yaml![1_u32], false)]
    #[test_case(&yaml![null], false)]
    #[test_case(&yaml![1.2_f32], false)]
    #[test_case(&yaml![{"key": "value"}], false)]
    #[test_case(&yaml!["string"], false)]
    fn test_is_array(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_array(value), expected_value);
    }

    #[test_case(&yaml![[0, 1, 2]], false)]
    #[test_case(&yaml![true], true)]
    #[test_case(&yaml![1_u32], false)]
    #[test_case(&yaml![null], false)]
    #[test_case(&yaml![1.2_f32], false)]
    #[test_case(&yaml![{"key": "value"}], false)]
    #[test_case(&yaml!["string"], false)]
    fn test_is_boolean(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_boolean(value), expected_value);
    }

    #[test_case(&yaml![[0, 1, 2]], false)]
    #[test_case(&yaml![true], false)]
    #[test_case(&yaml![1_u32], true)]
    #[test_case(&yaml![null], false)]
    #[test_case(&yaml![1.2_f32], false)]
    #[test_case(&yaml![{"key": "value"}], false)]
    #[test_case(&yaml!["string"], false)]
    fn test_is_integer(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_integer(value), expected_value);
    }

    #[test_case(&yaml![[0, 1, 2]], false)]
    #[test_case(&yaml![true], false)]
    #[test_case(&yaml![1_u32], false)]
    #[test_case(&yaml![null], true)]
    #[test_case(&yaml![1.2_f32], false)]
    #[test_case(&yaml![{"key": "value"}], false)]
    #[test_case(&yaml!["string"], false)]
    fn test_is_null(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_null(value), expected_value);
    }

    #[test_case(&yaml![[0, 1, 2]], false)]
    #[test_case(&yaml![true], false)]
    #[test_case(&yaml![1_u32], true)]
    #[test_case(&yaml![null], false)]
    #[test_case(&yaml![1.2_f32], true)]
    #[test_case(&yaml![{"key": "value"}], false)]
    #[test_case(&yaml!["string"], false)]
    fn test_is_number(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_number(value), expected_value);
    }

    #[test_case(&yaml![[0, 1, 2]], false)]
    #[test_case(&yaml![true], false)]
    #[test_case(&yaml![1_u32], false)]
    #[test_case(&yaml![null], false)]
    #[test_case(&yaml![1.2_f32], false)]
    #[test_case(&yaml![{"key": "value"}], true)]
    #[test_case(&yaml!["string"], false)]
    fn test_is_object(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_object(value), expected_value);
    }

    #[test_case(&yaml![[0, 1, 2]], false)]
    #[test_case(&yaml![true], false)]
    #[test_case(&yaml![1_u32], false)]
    #[test_case(&yaml![null], false)]
    #[test_case(&yaml![1.2_f32], false)]
    #[test_case(&yaml![{"key": "value"}], false)]
    #[test_case(&yaml!["string"], true)]
    fn test_is_string(value: &Value, expected_value: bool) {
        assert_eq!(JsonType::is_string(value), expected_value);
    }

    #[test_case(&yaml![[1]], &Some(vec![yaml![1]]))]
    #[test_case(&yaml![[1, "a"]], &Some(vec![yaml![1], yaml!["a"]]))]
    #[test_case(&yaml![null], &None)]
    fn test_as_array(value: &Value, expected_value: &Option<Vec<Value>>) {
        assert_eq!(&JsonType::as_array(value).map(|iterator| iterator.cloned().collect()), expected_value);
    }

    #[test_case(&yaml![true], Some(true))]
    #[test_case(&yaml![false], Some(false))]
    #[test_case(&yaml![1], None)]
    fn test_as_boolean(value: &Value, expected_value: Option<bool>) {
        assert_eq!(JsonType::as_boolean(value), expected_value);
    }

    #[test_case(&yaml![1], Some(1))]
    #[test_case(&yaml![1.2], None)]
    #[test_case(&yaml!["1"], None)]
    fn test_as_integer(value: &Value, expected_value: Option<i128>) {
        assert_eq!(JsonType::as_integer(value), expected_value);
    }

    #[test_case(&yaml![null], Some(()))]
    #[test_case(&yaml!["1"], None)]
    fn test_as_null(value: &Value, expected_value: Option<()>) {
        assert_eq!(JsonType::as_null(value), expected_value);
    }

    #[test_case(&yaml![1], Some(1_f64))]
    #[test_case(&yaml![1.2], Some(1.2))]
    #[test_case(&yaml!["1"], None)]
    fn test_as_number(value: &Value, expected_value: Option<f64>) {
        assert_eq!(JsonType::as_number(value), expected_value);
    }

    #[test_case(&yaml![1], &None)]
    #[test_case(&yaml![1.2], &None)]
    #[test_case(&yaml![{"1": 1}], &Some(yaml![{"1": 1}]))]
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

    #[test_case(&yaml![1], None)]
    #[test_case(&yaml![1.2], None)]
    #[test_case(&yaml!["1"], Some("1"))]
    fn test_as_string(value: &Value, expected_value: Option<&str>) {
        assert_eq!(JsonType::as_string(value), expected_value);
    }
}

#[cfg(test)]
mod tests_json_map {
    use crate::json_type::{JsonMapTrait, JsonType};
    use serde_yaml::Value;

    lazy_static! {
        static ref TESTING_MAP: Value = yaml![{"key1": {"key2": 1}}];
    }

    #[test]
    fn test_keys() {
        let key1: &Value = TESTING_MAP.get_attribute("key1").unwrap();
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
        let value = yaml![[
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
