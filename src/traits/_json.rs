use crate::{
    json_type::{to_rust_type, JsonMap, JsonMapTrait, JsonType},
    RustType, ThreadSafeJsonType,
};
use std::ops::Index;

impl Into<RustType> for json::JsonValue {
    fn into(self) -> RustType {
        to_rust_type(&self)
    }
}

impl<'json> JsonMapTrait<'json, json::JsonValue> for JsonMap<'json, json::JsonValue> {
    #[must_use]
    fn keys(&'json self) -> Box<dyn Iterator<Item = &str> + 'json> {
        Box::new(self.entries().map(|(key, _)| key))
    }

    #[must_use]
    fn values(&'json self) -> Box<dyn Iterator<Item = &json::JsonValue> + 'json> {
        Box::new(self.entries().map(|(_, value)| value))
    }

    #[must_use]
    fn items(&'json self) -> Box<dyn Iterator<Item = (&str, &json::JsonValue)> + 'json> {
        Box::new(self.entries())
    }
}

impl JsonType<json::JsonValue> for json::JsonValue {
    #[must_use]
    fn as_array<'json>(&'json self) -> Option<Box<dyn ExactSizeIterator<Item = &Self> + 'json>> {
        if self.is_array() {
            Some(Box::new(self.members()))
        } else {
            None
        }
    }

    #[must_use]
    fn as_boolean(&self) -> Option<bool> {
        self.as_bool()
    }

    #[must_use]
    fn as_integer(&self) -> Option<i128> {
        self.as_f64().and_then(
            // The ugly conversion here is needed because rust-json internally does not
            // distinguish integers from floats, which leads to have "1.2".as_i64() == 1
            |number| {
                if number.fract() == 0.0 {
                    #[allow(clippy::cast_possible_truncation)]
                    Some(i128::from(number.trunc() as i64))
                } else {
                    None
                }
            },
        )
    }

    #[must_use]
    fn as_null(&self) -> Option<()> {
        if self.is_null() {
            Some(())
        } else {
            None
        }
    }

    #[must_use]
    fn as_number(&self) -> Option<f64> {
        self.as_f64()
    }

    #[must_use]
    fn as_object(&self) -> Option<JsonMap<Self>>
    where
        for<'json> JsonMap<'json, Self>: JsonMapTrait<'json, Self>,
    {
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
        let extracted_value = self.index(attribute_name);
        if let Self::Null = extracted_value {
            None
        } else {
            Some(extracted_value)
        }
    }

    #[must_use]
    fn get_index(&self, index: usize) -> Option<&Self> {
        let extracted_value = self.index(index);
        if let Self::Null = extracted_value {
            None
        } else {
            Some(extracted_value)
        }
    }
}

impl dyn ThreadSafeJsonType<json::JsonValue> {}

#[cfg(test)]
mod tests_json_map_trait {
    use crate::{json_type::JsonMap, JsonMapTrait};

    lazy_static! {
        static ref TESTING_MAP: json::JsonValue = rust_json![{"k1": "v1", "k2": "v2"}];
    }

    #[test]
    fn keys() {
        let testing_map: &json::JsonValue = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).keys().collect::<Vec<_>>(), vec!["k1", "k2"]);
    }

    #[test]
    fn values() {
        let testing_map: &json::JsonValue = &TESTING_MAP;
        assert_eq!(
            JsonMap::new(testing_map).values().collect::<Vec<_>>(),
            vec![&json::JsonValue::from("v1"), &json::JsonValue::from("v2")]
        );
    }

    #[test]
    fn items() {
        let testing_map: &json::JsonValue = &TESTING_MAP;
        assert_eq!(
            JsonMap::new(testing_map).items().collect::<Vec<_>>(),
            vec![("k1", &json::JsonValue::from("v1")), ("k2", &json::JsonValue::from("v2"))]
        );
    }
}

#[cfg(test)]
mod tests_primitive_type_trait {
    use crate::json_type::{JsonType, PrimitiveType};
    use test_case::test_case;

    #[test_case(&rust_json![[]], PrimitiveType::Array)]
    #[test_case(&rust_json![true], PrimitiveType::Boolean)]
    #[test_case(&rust_json![1], PrimitiveType::Integer)]
    #[test_case(&rust_json![null], PrimitiveType::Null)]
    #[test_case(&rust_json![1.2], PrimitiveType::Number)]
    #[test_case(&rust_json![{"prop": "value"}], PrimitiveType::Object)]
    #[test_case(&rust_json!["string"], PrimitiveType::String)]
    fn test_primitive_type(value: &json::JsonValue, expected_value: PrimitiveType) {
        assert_eq!(JsonType::primitive_type(value), expected_value);
    }

    #[test_case(&rust_json![{"present": 1}], "present", &Some(rust_json![1]))]
    #[test_case(&rust_json![{"present": 1}], "not-present", &None)]
    fn test_get_attribute(value: &json::JsonValue, attribute_name: &str, expected_value: &Option<json::JsonValue>) {
        assert_eq!(JsonType::get_attribute(value, attribute_name), expected_value.as_ref());
    }

    #[test_case(&rust_json![[0, 1, 2]], 1, &Some(rust_json![1]))]
    #[test_case(&rust_json![[0, 1, 2]], 4, &None)]
    fn test_get_index(value: &json::JsonValue, index: usize, expected_value: &Option<json::JsonValue>) {
        assert_eq!(JsonType::get_index(value, index), expected_value.as_ref());
    }

    #[test_case(&rust_json![{"present": 1}], "present", true)]
    #[test_case(&rust_json![{"present": 1}], "not-present", false)]
    #[test_case(&rust_json![[1, 2, 3]], "not-present", false)]
    fn test_has_attribute(value: &json::JsonValue, attr_name: &str, expected_value: bool) {
        assert_eq!(JsonType::has_attribute(value, attr_name), expected_value);
    }

    #[test_case(&rust_json![[0, 1, 2]], true)]
    #[test_case(&rust_json![true], false)]
    #[test_case(&rust_json![1_u32], false)]
    #[test_case(&rust_json![null], false)]
    #[test_case(&rust_json![1.2_f32], false)]
    #[test_case(&rust_json![{"key": "value"}], false)]
    #[test_case(&rust_json!["string"], false)]
    fn test_is_array(value: &json::JsonValue, expected_value: bool) {
        assert_eq!(JsonType::is_array(value), expected_value);
    }

    #[test_case(&rust_json![[0, 1, 2]], false)]
    #[test_case(&rust_json![true], true)]
    #[test_case(&rust_json![1_u32], false)]
    #[test_case(&rust_json![null], false)]
    #[test_case(&rust_json![1.2_f32], false)]
    #[test_case(&rust_json![{"key": "value"}], false)]
    #[test_case(&rust_json!["string"], false)]
    fn test_is_boolean(value: &json::JsonValue, expected_value: bool) {
        assert_eq!(JsonType::is_boolean(value), expected_value);
    }

    #[test_case(&rust_json![[0, 1, 2]], false)]
    #[test_case(&rust_json![true], false)]
    #[test_case(&rust_json![1_u32], true)]
    #[test_case(&rust_json![null], false)]
    #[test_case(&rust_json![1.2_f32], false)]
    #[test_case(&rust_json![{"key": "value"}], false)]
    #[test_case(&rust_json!["string"], false)]
    fn test_is_integer(value: &json::JsonValue, expected_value: bool) {
        assert_eq!(JsonType::is_integer(value), expected_value);
    }

    #[test_case(&rust_json![[0, 1, 2]], false)]
    #[test_case(&rust_json![true], false)]
    #[test_case(&rust_json![1_u32], false)]
    #[test_case(&rust_json![null], true)]
    #[test_case(&rust_json![1.2_f32], false)]
    #[test_case(&rust_json![{"key": "value"}], false)]
    #[test_case(&rust_json!["string"], false)]
    fn test_is_null(value: &json::JsonValue, expected_value: bool) {
        assert_eq!(JsonType::is_null(value), expected_value);
    }

    #[test_case(&rust_json![[0, 1, 2]], false)]
    #[test_case(&rust_json![true], false)]
    #[test_case(&rust_json![1_u32], true)]
    #[test_case(&rust_json![null], false)]
    #[test_case(&rust_json![1.2_f32], true)]
    #[test_case(&rust_json![{"key": "value"}], false)]
    #[test_case(&rust_json!["string"], false)]
    fn test_is_number(value: &json::JsonValue, expected_value: bool) {
        assert_eq!(JsonType::is_number(value), expected_value);
    }

    #[test_case(&rust_json![[0, 1, 2]], false)]
    #[test_case(&rust_json![true], false)]
    #[test_case(&rust_json![1_u32], false)]
    #[test_case(&rust_json![null], false)]
    #[test_case(&rust_json![1.2_f32], false)]
    #[test_case(&rust_json![{"key": "value"}], true)]
    #[test_case(&rust_json!["string"], false)]
    fn test_is_object(value: &json::JsonValue, expected_value: bool) {
        assert_eq!(JsonType::is_object(value), expected_value);
    }

    #[test_case(&rust_json![[0, 1, 2]], false)]
    #[test_case(&rust_json![true], false)]
    #[test_case(&rust_json![1_u32], false)]
    #[test_case(&rust_json![null], false)]
    #[test_case(&rust_json![1.2_f32], false)]
    #[test_case(&rust_json![{"key": "value"}], false)]
    #[test_case(&rust_json!["string"], true)]
    fn test_is_string(value: &json::JsonValue, expected_value: bool) {
        assert_eq!(JsonType::is_string(value), expected_value);
    }

    #[test_case(&rust_json![[1]], &Some(vec![rust_json![1]]))]
    #[test_case(&rust_json![[1, "a"]], &Some(vec![rust_json![1], rust_json!["a"]]))]
    #[test_case(&rust_json![null], &None)]
    fn test_as_array(value: &json::JsonValue, expected_value: &Option<Vec<json::JsonValue>>) {
        assert_eq!(&JsonType::as_array(value).map(|iterator| iterator.cloned().collect()), expected_value);
    }

    #[test_case(&rust_json![true], Some(true))]
    #[test_case(&rust_json![false], Some(false))]
    #[test_case(&rust_json![1], None)]
    fn test_as_boolean(value: &json::JsonValue, expected_value: Option<bool>) {
        assert_eq!(JsonType::as_boolean(value), expected_value);
    }

    #[test_case(&rust_json![1], Some(1))]
    #[test_case(&rust_json![1.2], None)]
    #[test_case(&rust_json!["1"], None)]
    fn test_as_integer(value: &json::JsonValue, expected_value: Option<i128>) {
        assert_eq!(JsonType::as_integer(value), expected_value);
    }

    #[test_case(&rust_json![null], Some(()))]
    #[test_case(&rust_json!["1"], None)]
    fn test_as_null(value: &json::JsonValue, expected_value: Option<()>) {
        assert_eq!(JsonType::as_null(value), expected_value);
    }

    #[test_case(&rust_json![1], Some(1_f64))]
    #[test_case(&rust_json![1.2], Some(1.2))]
    #[test_case(&rust_json!["1"], None)]
    fn test_as_number(value: &json::JsonValue, expected_value: Option<f64>) {
        assert_eq!(JsonType::as_number(value), expected_value);
    }

    #[test_case(&rust_json![1], &None)]
    #[test_case(&rust_json![1.2], &None)]
    #[test_case(&rust_json![{"1": 1}], &Some(rust_json![{"1": 1}]))]
    fn test_as_object(value: &json::JsonValue, expected_value: &Option<json::JsonValue>) {
        use std::ops::Deref;

        assert_eq!(
            match JsonType::as_object(value) {
                Some(ref v) => Some(v.deref()),
                None => None,
            },
            expected_value.as_ref(),
        );
    }

    #[test_case(&rust_json![1], None)]
    #[test_case(&rust_json![1.2], None)]
    #[test_case(&rust_json!["1"], Some("1"))]
    fn test_as_string(value: &json::JsonValue, expected_value: Option<&str>) {
        assert_eq!(JsonType::as_string(value), expected_value);
    }
}

#[cfg(test)]
mod json_map_tests {
    use crate::json_type::{JsonMapTrait, JsonType};

    lazy_static! {
        static ref TESTING_MAP: json::JsonValue = rust_json![{"key1": {"key2": 1}}];
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
            vec![format!("{:?}", json::JsonValue::from(1))],
        );
    }

    #[test]
    fn test_items() {
        let key1 = TESTING_MAP.get_attribute("key1").unwrap();
        assert_eq!(
            JsonType::as_object(key1).unwrap().items().map(|(k, v)| format!("{} -> {:?}", k, v)).collect::<Vec<_>>(),
            vec![format!("key2 -> {:?}", json::JsonValue::from(1))],
        );
    }
}
