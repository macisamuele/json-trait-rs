use crate::json_type::{JsonMap, JsonMapTrait, JsonType};
use serde_yaml;

impl<'json> JsonMapTrait<'json, serde_yaml::Value> for JsonMap<'json, serde_yaml::Value> {
    #[inline]
    fn keys(&'json self) -> Box<ExactSizeIterator<Item = &str> + 'json> {
        if let Some(obj) = self.as_mapping() {
            Box::new(obj.iter().map(|(key, _)| (key.as_str().unwrap())))
        } else {
            #[allow(unsafe_code)]
            unsafe {
                unreachable::unreachable()
            }
        }
    }

    #[inline]
    fn values(&'json self) -> Box<ExactSizeIterator<Item = &serde_yaml::Value> + 'json> {
        if let Some(obj) = self.as_mapping() {
            Box::new(obj.iter().map(|(_, value)| value))
        } else {
            #[allow(unsafe_code)]
            unsafe {
                unreachable::unreachable()
            }
        }
    }

    #[inline]
    fn items(&'json self) -> Box<ExactSizeIterator<Item = (&str, &serde_yaml::Value)> + 'json> {
        if let Some(obj) = self.as_mapping() {
            Box::new(obj.iter().map(|(key, value)| (key.as_str().unwrap(), value)))
        } else {
            #[allow(unsafe_code)]
            unsafe {
                unreachable::unreachable()
            }
        }
    }
}

impl<'json> JsonType<'json> for serde_yaml::Value {
    fn as_array(&'json self) -> Option<Box<ExactSizeIterator<Item = &Self> + 'json>> {
        if let Some(vec) = self.as_sequence() {
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

    fn as_object(&'json self) -> Option<JsonMap<Self>> {
        if self.as_mapping().is_some() {
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
mod tests_yaml_map_trait {
    use crate::json_type::{JsonMap, JsonMapTrait};
    use serde_yaml;

    lazy_static! {
        static ref TESTING_MAP: serde_yaml::Value = yaml![{"k1": "v1", "k2": "v2"}];
    }

    #[test]
    fn keys() {
        let testing_map: &serde_yaml::Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).keys().collect::<Vec<_>>(), vec!["k1", "k2"]);
    }

    #[test]
    fn values() {
        let testing_map: &serde_yaml::Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).values().collect::<Vec<_>>(), vec![&yaml!["v1"], &yaml!["v2"]]);
    }

    #[test]
    fn items() {
        let testing_map: &serde_yaml::Value = &TESTING_MAP;
        assert_eq!(JsonMap::new(testing_map).items().collect::<Vec<_>>(), vec![("k1", &yaml!["v1"]), ("k2", &yaml!["v2"])]);
    }
}

#[cfg(test)]
mod tests_primitive_type_trait {
    use crate::{
        index::Index,
        json_type::{EnumJsonType, JsonType},
    };
    use test_case_derive::test_case;

    #[test_case(yaml![[]], EnumJsonType::Array)]
    #[test_case(yaml![true], EnumJsonType::Boolean)]
    #[test_case(yaml![1], EnumJsonType::Integer)]
    #[test_case(yaml![null], EnumJsonType::Null)]
    #[test_case(yaml![1.2], EnumJsonType::Number)]
    #[test_case(yaml![{"prop": "value"}], EnumJsonType::Object)]
    #[test_case(yaml!["string"], EnumJsonType::String)]
    fn test_primitive_type(value: serde_yaml::Value, expected_value: EnumJsonType) {
        assert_eq!(JsonType::primitive_type(&value), expected_value);
    }

    #[test_case(yaml![{"present": 1}], "present", Some(&yaml![1]))]
    #[test_case(yaml![{"present": 1}], "not-present", None)]
    fn test_get_attribute(value: serde_yaml::Value, attribute_name: &str, expected_value: Option<&serde_yaml::Value>) {
        assert_eq!(JsonType::get_attribute(&value, attribute_name), expected_value);
    }

    #[test_case(yaml![[0, 1, 2]], 1, Some(&yaml![1]))]
    #[test_case(yaml![[0, 1, 2]], 4, None)]
    fn test_get_index(value: serde_yaml::Value, index: usize, expected_value: Option<&serde_yaml::Value>) {
        assert_eq!(JsonType::get_index(&value, index), expected_value);
    }

    #[test_case(&yaml![{"present": 1}], "present", Some(&yaml![1]))]
    #[test_case(&yaml![{"present": 1}], "not-present", None)]
    #[test_case(&yaml![[0, 1, 2]], 1, Some(&yaml![1]))]
    #[test_case(&yaml![[0, 1, 2]], 4, None)]
    fn test_get<'json, I: Index<'json, serde_yaml::Value>>(value: &'json serde_yaml::Value, index_value: I, expected_value: Option<&'json serde_yaml::Value>) {
        assert_eq!(JsonType::get(value, index_value), expected_value);
    }

    #[test_case(yaml![{"present": 1}], "present", true)]
    #[test_case(yaml![{"present": 1}], "not-present", false)]
    #[test_case(yaml![[1, 2, 3]], "not-present", false)]
    fn test_has_attribute(value: serde_yaml::Value, attr_name: &str, expected_value: bool) {
        assert_eq!(JsonType::has_attribute(&value, attr_name), expected_value);
    }

    #[test_case(yaml![[0, 1, 2]], true)]
    #[test_case(yaml![true], false)]
    #[test_case(yaml![1_u32], false)]
    #[test_case(yaml![null], false)]
    #[test_case(yaml![1.2_f32], false)]
    #[test_case(yaml![{"key": "value"}], false)]
    #[test_case(yaml!["string"], false)]
    fn test_is_array(value: serde_yaml::Value, expected_value: bool) {
        assert_eq!(JsonType::is_array(&value), expected_value);
    }

    #[test_case(yaml![[0, 1, 2]], false)]
    #[test_case(yaml![true], true)]
    #[test_case(yaml![1_u32], false)]
    #[test_case(yaml![null], false)]
    #[test_case(yaml![1.2_f32], false)]
    #[test_case(yaml![{"key": "value"}], false)]
    #[test_case(yaml!["string"], false)]
    fn test_is_boolean(value: serde_yaml::Value, expected_value: bool) {
        assert_eq!(JsonType::is_boolean(&value), expected_value);
    }

    #[test_case(yaml![[0, 1, 2]], false)]
    #[test_case(yaml![true], false)]
    #[test_case(yaml![1_u32], true)]
    #[test_case(yaml![null], false)]
    #[test_case(yaml![1.2_f32], false)]
    #[test_case(yaml![{"key": "value"}], false)]
    #[test_case(yaml!["string"], false)]
    fn test_is_integer(value: serde_yaml::Value, expected_value: bool) {
        assert_eq!(JsonType::is_integer(&value), expected_value);
    }

    #[test_case(yaml![[0, 1, 2]], false)]
    #[test_case(yaml![true], false)]
    #[test_case(yaml![1_u32], false)]
    #[test_case(yaml![null], true)]
    #[test_case(yaml![1.2_f32], false)]
    #[test_case(yaml![{"key": "value"}], false)]
    #[test_case(yaml!["string"], false)]
    fn test_is_null(value: serde_yaml::Value, expected_value: bool) {
        assert_eq!(JsonType::is_null(&value), expected_value);
    }

    #[test_case(yaml![[0, 1, 2]], false)]
    #[test_case(yaml![true], false)]
    #[test_case(yaml![1_u32], true)]
    #[test_case(yaml![null], false)]
    #[test_case(yaml![1.2_f32], true)]
    #[test_case(yaml![{"key": "value"}], false)]
    #[test_case(yaml!["string"], false)]
    fn test_is_number(value: serde_yaml::Value, expected_value: bool) {
        assert_eq!(JsonType::is_number(&value), expected_value);
    }

    #[test_case(yaml![[0, 1, 2]], false)]
    #[test_case(yaml![true], false)]
    #[test_case(yaml![1_u32], false)]
    #[test_case(yaml![null], false)]
    #[test_case(yaml![1.2_f32], false)]
    #[test_case(yaml![{"key": "value"}], true)]
    #[test_case(yaml!["string"], false)]
    fn test_is_object(value: serde_yaml::Value, expected_value: bool) {
        assert_eq!(JsonType::is_object(&value), expected_value);
    }

    #[test_case(yaml![[0, 1, 2]], false)]
    #[test_case(yaml![true], false)]
    #[test_case(yaml![1_u32], false)]
    #[test_case(yaml![null], false)]
    #[test_case(yaml![1.2_f32], false)]
    #[test_case(yaml![{"key": "value"}], false)]
    #[test_case(yaml!["string"], true)]
    fn test_is_string(value: serde_yaml::Value, expected_value: bool) {
        assert_eq!(JsonType::is_string(&value), expected_value);
    }

    #[test_case(yaml![[1]], Some(vec![&yaml![1]]))]
    #[test_case(yaml![[1, "a"]], Some(vec![&yaml![1], &yaml!["a"]]))]
    #[test_case(yaml![null], None)]
    fn test_as_array(value: serde_yaml::Value, expected_value: Option<Vec<&serde_yaml::Value>>) {
        assert_eq!(JsonType::as_array(&value).and_then(|iterator| Some(iterator.collect())), expected_value);
    }

    #[test_case(yaml![true], Some(true))]
    #[test_case(yaml![false], Some(false))]
    #[test_case(yaml![1], None)]
    fn test_as_boolean(value: serde_yaml::Value, expected_value: Option<bool>) {
        assert_eq!(JsonType::as_boolean(&value), expected_value);
    }

    #[test_case(yaml![1], Some(1))]
    #[test_case(yaml![1.2], None)]
    #[test_case(yaml!["1"], None)]
    fn test_as_integer(value: serde_yaml::Value, expected_value: Option<i128>) {
        assert_eq!(JsonType::as_integer(&value), expected_value);
    }

    #[test_case(yaml![null], Some(()))]
    #[test_case(yaml!["1"], None)]
    fn test_as_null(value: serde_yaml::Value, expected_value: Option<()>) {
        assert_eq!(JsonType::as_null(&value), expected_value);
    }

    #[test_case(yaml![1], Some(1_f64))]
    #[test_case(yaml![1.2], Some(1.2))]
    #[test_case(yaml!["1"], None)]
    fn test_as_number(value: serde_yaml::Value, expected_value: Option<f64>) {
        assert_eq!(JsonType::as_number(&value), expected_value);
    }

    #[test_case(yaml![1], None)]
    #[test_case(yaml![1.2], None)]
    #[test_case(yaml![{"1": 1}], Some(&yaml![{"1": 1}]))]
    fn test_as_object(value: serde_yaml::Value, expected_value: Option<&serde_yaml::Value>) {
        use std::ops::Deref;

        let option_as_object = JsonType::as_object(&value);

        assert_eq!(option_as_object.is_some(), expected_value.is_some());

        if let Some(as_object) = option_as_object {
            assert_eq!(as_object.deref(), expected_value.unwrap());
        }
    }

    #[test_case(yaml![1], None)]
    #[test_case(yaml![1.2], None)]
    #[test_case(yaml!["1"], Some("1"))]
    fn test_as_string(value: serde_yaml::Value, expected_value: Option<&str>) {
        assert_eq!(JsonType::as_string(&value), expected_value);
    }
}
