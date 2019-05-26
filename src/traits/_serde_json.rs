use crate::json_type::{JsonMap, JsonMapTrait, JsonType};
use serde_json;

impl<'json> JsonMapTrait<'json, serde_json::Value> for JsonMap<'json, serde_json::Value> {
    #[inline]
    fn keys(&'json self) -> Box<ExactSizeIterator<Item = &str> + 'json> {
        if let Some(obj) = self.as_object() {
            Box::new(obj.keys().map(String::as_str))
        } else {
            #[allow(unsafe_code)]
            unsafe {
                unreachable::unreachable()
            }
        }
    }

    #[inline]
    fn values(&'json self) -> Box<ExactSizeIterator<Item = &serde_json::Value> + 'json> {
        if let Some(obj) = self.as_object() {
            Box::new(obj.values())
        } else {
            #[allow(unsafe_code)]
            unsafe {
                unreachable::unreachable()
            }
        }
    }

    #[inline]
    fn items(&'json self) -> Box<ExactSizeIterator<Item = (&str, &serde_json::Value)> + 'json> {
        if let Some(obj) = self.as_object() {
            Box::new(obj.iter().map(|(key, value)| (key.as_str(), value)))
        } else {
            #[allow(unsafe_code)]
            unsafe {
                unreachable::unreachable()
            }
        }
    }
}

impl JsonType for serde_json::Value {
    fn as_array<'json>(&'json self) -> Option<Box<ExactSizeIterator<Item = &Self> + 'json>> {
        if let Some(vec) = self.as_array() {
            Some(Box::new(vec.iter()))
        } else {
            None
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        self.as_bool()
    }

    fn as_integer(&self) -> Option<i128> {
        if let Some(value) = self.as_i64() {
            Some(i128::from(value))
        } else {
            None
        }
    }

    fn as_null(&self) -> Option<()> {
        self.as_null()
    }

    fn as_number(&self) -> Option<f64> {
        self.as_f64()
    }

    fn as_object<'json>(&'json self) -> Option<JsonMap<Self>>
    where
        JsonMap<'json, Self>: JsonMapTrait<'json, Self>,
    {
        if self.as_object().is_some() {
            Some(JsonMap::new(self))
        } else {
            None
        }
    }

    fn as_string(&self) -> Option<&str> {
        self.as_str()
    }

    fn get_attribute<R: AsRef<str>>(&self, attribute_name: R) -> Option<&Self> {
        self.get(attribute_name.as_ref())
    }

    fn get_index(&self, index: usize) -> Option<&Self> {
        self.get(index)
    }

    fn has_attribute(&self, attribute_name: &str) -> bool {
        self.get(attribute_name).is_some()
    }
}

#[cfg(test)]
mod tests_json_map_trait {
    use crate::json_type::{JsonMap, JsonMapTrait};
    use serde_json;

    lazy_static! {
        static ref TESTING_MAP: serde_json::Value = json![{"k1": "v1", "k2": "v2"}];
    }

    #[test]
    fn keys() {
        let testing_map: &serde_json::Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).keys().collect::<Vec<_>>(), vec!["k1", "k2"]);
    }

    #[test]
    fn values() {
        let testing_map: &serde_json::Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).values().collect::<Vec<_>>(), vec![&json!["v1"], &json!["v2"]]);
    }

    #[test]
    fn items() {
        let testing_map: &serde_json::Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).items().collect::<Vec<_>>(), vec![("k1", &json!["v1"]), ("k2", &json!["v2"])]);
    }
}

#[cfg(test)]
mod tests_primitive_type_trait {
    use crate::{
        index::Index,
        json_type::{EnumJsonType, JsonType},
    };
    use test_case_derive::test_case;

    #[test_case(json![[]], EnumJsonType::Array)]
    #[test_case(json![true], EnumJsonType::Boolean)]
    #[test_case(json![1], EnumJsonType::Integer)]
    #[test_case(json![null], EnumJsonType::Null)]
    #[test_case(json![1.2], EnumJsonType::Number)]
    #[test_case(json![{"prop": "value"}], EnumJsonType::Object)]
    #[test_case(json!["string"], EnumJsonType::String)]
    fn test_primitive_type(value: serde_json::Value, expected_value: EnumJsonType) {
        assert_eq!(JsonType::primitive_type(&value), expected_value);
    }

    #[test_case(json![{"present": 1}], "present", Some(&json![1]))]
    #[test_case(json![{"present": 1}], "not-present", None)]
    fn test_get_attribute(value: serde_json::Value, attribute_name: &str, expected_value: Option<&serde_json::Value>) {
        assert_eq!(JsonType::get_attribute(&value, attribute_name), expected_value);
    }

    #[test_case(json![[0, 1, 2]], 1, Some(&json![1]))]
    #[test_case(json![[0, 1, 2]], 4, None)]
    fn test_get_index(value: serde_json::Value, index: usize, expected_value: Option<&serde_json::Value>) {
        assert_eq!(JsonType::get_index(&value, index), expected_value);
    }

    #[test_case(&json![{"present": 1}], "present", Some(&json![1]))]
    #[test_case(&json![{"present": 1}], "not-present", None)]
    #[test_case(&json![[0, 1, 2]], 1, Some(&json![1]))]
    #[test_case(&json![[0, 1, 2]], 4, None)]
    fn test_get<'json, I: Index<serde_json::Value>>(value: &'json serde_json::Value, index_value: I, expected_value: Option<&'json serde_json::Value>) {
        assert_eq!(JsonType::get(value, index_value), expected_value);
    }

    #[test_case(json![{"present": 1}], "present", true)]
    #[test_case(json![{"present": 1}], "not-present", false)]
    #[test_case(json![[1, 2, 3]], "not-present", false)]
    fn test_has_attribute(value: serde_json::Value, attr_name: &str, expected_value: bool) {
        assert_eq!(JsonType::has_attribute(&value, attr_name), expected_value);
    }

    #[test_case(json![[0, 1, 2]], true)]
    #[test_case(json![true], false)]
    #[test_case(json![1_u32], false)]
    #[test_case(json![null], false)]
    #[test_case(json![1.2_f32], false)]
    #[test_case(json![{"key": "value"}], false)]
    #[test_case(json!["string"], false)]
    fn test_is_array(value: serde_json::Value, expected_value: bool) {
        assert_eq!(JsonType::is_array(&value), expected_value);
    }

    #[test_case(json![[0, 1, 2]], false)]
    #[test_case(json![true], true)]
    #[test_case(json![1_u32], false)]
    #[test_case(json![null], false)]
    #[test_case(json![1.2_f32], false)]
    #[test_case(json![{"key": "value"}], false)]
    #[test_case(json!["string"], false)]
    fn test_is_boolean(value: serde_json::Value, expected_value: bool) {
        assert_eq!(JsonType::is_boolean(&value), expected_value);
    }

    #[test_case(json![[0, 1, 2]], false)]
    #[test_case(json![true], false)]
    #[test_case(json![1_u32], true)]
    #[test_case(json![null], false)]
    #[test_case(json![1.2_f32], false)]
    #[test_case(json![{"key": "value"}], false)]
    #[test_case(json!["string"], false)]
    fn test_is_integer(value: serde_json::Value, expected_value: bool) {
        assert_eq!(JsonType::is_integer(&value), expected_value);
    }

    #[test_case(json![[0, 1, 2]], false)]
    #[test_case(json![true], false)]
    #[test_case(json![1_u32], false)]
    #[test_case(json![null], true)]
    #[test_case(json![1.2_f32], false)]
    #[test_case(json![{"key": "value"}], false)]
    #[test_case(json!["string"], false)]
    fn test_is_null(value: serde_json::Value, expected_value: bool) {
        assert_eq!(JsonType::is_null(&value), expected_value);
    }

    #[test_case(json![[0, 1, 2]], false)]
    #[test_case(json![true], false)]
    #[test_case(json![1_u32], true)]
    #[test_case(json![null], false)]
    #[test_case(json![1.2_f32], true)]
    #[test_case(json![{"key": "value"}], false)]
    #[test_case(json!["string"], false)]
    fn test_is_number(value: serde_json::Value, expected_value: bool) {
        assert_eq!(JsonType::is_number(&value), expected_value);
    }

    #[test_case(json![[0, 1, 2]], false)]
    #[test_case(json![true], false)]
    #[test_case(json![1_u32], false)]
    #[test_case(json![null], false)]
    #[test_case(json![1.2_f32], false)]
    #[test_case(json![{"key": "value"}], true)]
    #[test_case(json!["string"], false)]
    fn test_is_object(value: serde_json::Value, expected_value: bool) {
        assert_eq!(JsonType::is_object(&value), expected_value);
    }

    #[test_case(json![[0, 1, 2]], false)]
    #[test_case(json![true], false)]
    #[test_case(json![1_u32], false)]
    #[test_case(json![null], false)]
    #[test_case(json![1.2_f32], false)]
    #[test_case(json![{"key": "value"}], false)]
    #[test_case(json!["string"], true)]
    fn test_is_string(value: serde_json::Value, expected_value: bool) {
        assert_eq!(JsonType::is_string(&value), expected_value);
    }

    #[test_case(json![[1]], Some(vec![&json![1]]))]
    #[test_case(json![[1, "a"]], Some(vec![&json![1], &json!["a"]]))]
    #[test_case(json![null], None)]
    fn test_as_array(value: serde_json::Value, expected_value: Option<Vec<&serde_json::Value>>) {
        assert_eq!(JsonType::as_array(&value).and_then(|iterator| Some(iterator.collect())), expected_value);
    }

    #[test_case(json![true], Some(true))]
    #[test_case(json![false], Some(false))]
    #[test_case(json![1], None)]
    fn test_as_boolean(value: serde_json::Value, expected_value: Option<bool>) {
        assert_eq!(JsonType::as_boolean(&value), expected_value);
    }

    #[test_case(json![1], Some(1))]
    #[test_case(json![1.2], None)]
    #[test_case(json!["1"], None)]
    fn test_as_integer(value: serde_json::Value, expected_value: Option<i128>) {
        assert_eq!(JsonType::as_integer(&value), expected_value);
    }

    #[test_case(json![null], Some(()))]
    #[test_case(json!["1"], None)]
    fn test_as_null(value: serde_json::Value, expected_value: Option<()>) {
        assert_eq!(JsonType::as_null(&value), expected_value);
    }

    #[test_case(json![1], Some(1_f64))]
    #[test_case(json![1.2], Some(1.2))]
    #[test_case(json!["1"], None)]
    fn test_as_number(value: serde_json::Value, expected_value: Option<f64>) {
        assert_eq!(JsonType::as_number(&value), expected_value);
    }

    #[test_case(json![1], None)]
    #[test_case(json![1.2], None)]
    #[test_case(json![{"1": 1}], Some(&json![{"1": 1}]))]
    fn test_as_object(value: serde_json::Value, expected_value: Option<&serde_json::Value>) {
        use std::ops::Deref;

        let option_as_object = JsonType::as_object(&value);

        assert_eq!(option_as_object.is_some(), expected_value.is_some());

        if let Some(as_object) = option_as_object {
            assert_eq!(as_object.deref(), expected_value.unwrap());
        }
    }

    #[test_case(json![1], None)]
    #[test_case(json![1.2], None)]
    #[test_case(json!["1"], Some("1"))]
    fn test_as_string(value: serde_json::Value, expected_value: Option<&str>) {
        assert_eq!(JsonType::as_string(&value), expected_value);
    }
}
