use crate::{JsonMap, JsonMapTrait, JsonType};
#[cfg(test)]
use pyo3::Python;
use pyo3::{
    types::{PyAny, PyDict, PySequence},
    ObjectProtocol, PyTryInto,
};
use std::{convert::TryInto, ops::Deref};

impl<'json> JsonMapTrait<'json, PyAny> for JsonMap<'json, PyAny> {
    #[must_use]
    fn keys(&'json self) -> Box<dyn Iterator<Item = &str> + 'json> {
        match PyTryInto::<PyDict>::try_into(self.deref()) {
            Ok(python_dict) => Box::new(python_dict.iter().filter_map(|(k, _)| k.as_string())),
            Err(_) => Box::new(Vec::with_capacity(0).into_iter()),
        }
    }

    #[must_use]
    fn values(&'json self) -> Box<dyn Iterator<Item = &PyAny> + 'json> {
        match PyTryInto::<PyDict>::try_into(self.deref()) {
            Ok(python_dict) => Box::new(python_dict.iter().map(|(_, v)| v)),
            Err(_) => Box::new(Vec::with_capacity(0).into_iter()),
        }
    }

    #[must_use]
    fn items(&'json self) -> Box<dyn Iterator<Item = (&str, &PyAny)> + 'json> {
        match PyTryInto::<PyDict>::try_into(self.deref()) {
            Ok(python_dict) => Box::new(python_dict.iter().filter_map(|(k, v)| k.as_string().map(|k_string| (k_string, v)).or(None))),
            Err(_) => Box::new(Vec::with_capacity(0).into_iter()),
        }
    }
}

impl JsonType<PyAny> for PyAny {
    #[must_use]
    fn as_array<'json>(&'json self) -> Option<Box<dyn ExactSizeIterator<Item = &Self> + 'json>> {
        match PyTryInto::<PySequence>::try_into(self) {
            Err(_) => None,
            Ok(py_sequence) => match py_sequence.iter() {
                Err(_) => None,
                Ok(iterator) => {
                    if self.is_string() {
                        None
                    } else {
                        Some(Box::new(iterator.filter_map(Result::ok).collect::<Vec<_>>().into_iter()))
                    }
                }
            },
        }
    }

    #[must_use]
    fn as_boolean(&self) -> Option<bool> {
        self.extract().ok()
    }

    #[must_use]
    fn as_integer(&self) -> Option<i128> {
        self.extract().ok().and_then(|value| {
            // In python `assert isinstance(True, int) is True` is correct
            // So if we're able to convert the instance to a i128 instance then we need
            // to verify that we did not start from a boolean instance
            if self.is_boolean() {
                None
            } else {
                Some(value)
            }
        })
    }

    #[must_use]
    fn as_null(&self) -> Option<()> {
        if self.is_none() {
            Some(())
        } else {
            None
        }
    }

    #[must_use]
    fn as_number(&self) -> Option<f64> {
        self.extract().ok().and_then(|value| {
            // pyo3 is able to convert a boolean value into a f64 instance
            // So if we're converted the PyAny instance into a f64 instance then we need
            // to verify that we did not start from a boolean instance
            if self.is_boolean() {
                None
            } else {
                Some(value)
            }
        })
    }

    #[must_use]
    fn as_object(&self) -> Option<JsonMap<Self>>
    where
        for<'json> JsonMap<'json, Self>: JsonMapTrait<'json, Self>,
    {
        PyTryInto::<PyDict>::try_into(self).ok().map(|_| JsonMap::new(self))
    }

    #[must_use]
    fn as_string(&self) -> Option<&str> {
        self.extract().ok()
    }

    #[must_use]
    fn get_attribute(&self, attribute_name: &str) -> Option<&Self> {
        if let Ok(python_dict) = PyTryInto::<PyDict>::try_into(self) {
            return (python_dict as &PyDict).get_item(attribute_name);
        }
        None
    }

    #[must_use]
    fn get_index(&self, index: usize) -> Option<&Self> {
        if let Ok(idx) = TryInto::<isize>::try_into(index) {
            if let Ok(python_sequence) = PyTryInto::<PySequence>::try_into(self) {
                return python_sequence.get_item(idx).ok();
            }
        }
        None
    }
}

#[cfg(test)]
fn perform_python_check(python_code_string: &str, check: impl Fn(&PyAny) -> ()) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    check(py.eval(python_code_string, None, None).unwrap())
}

#[cfg(test)]
mod tests_json_map_trait {
    use crate::{json_type::JsonMap, traits::_pyo3::perform_python_check, JsonMapTrait};
    use std::collections::HashSet;

    lazy_static! {
        static ref PYTHON_TESTING_MAP_STR: &'static str = "{'k1': 'v1', 'k2': 'v2'}";
    }

    #[test]
    fn keys() {
        perform_python_check(&PYTHON_TESTING_MAP_STR, |python_object_ref| {
            // HashSet needed as python does not guarantee ordering of keys, or anyway we should not care about ordering
            assert_eq!(
                JsonMap::new(python_object_ref).keys().collect::<HashSet<_>>(),
                vec!["k1", "k2"].into_iter().collect::<HashSet<_>>()
            );
        });
    }

    #[test]
    fn values() {
        perform_python_check(&PYTHON_TESTING_MAP_STR, |python_object_ref| {
            // HashSet needed as python does not guarantee ordering of keys, or anyway we should not care about ordering
            assert_eq!(
                JsonMap::new(python_object_ref).values().map(|value| format!("{}", value)).collect::<HashSet<_>>(),
                vec!["v1".to_string(), "v2".to_string()].into_iter().collect::<HashSet<_>>()
            );
        });
    }

    #[test]
    fn items() {
        perform_python_check(&PYTHON_TESTING_MAP_STR, |python_object_ref| {
            // HashSet needed as python does not guarantee ordering of keys, or anyway we should not care about ordering
            assert_eq!(
                JsonMap::new(python_object_ref)
                    .items()
                    .map(|(key, value)| (key, format!("{}", value)))
                    .collect::<HashSet<_>>(),
                vec![("k1", "v1".to_string()), ("k2", "v2".to_string()),].into_iter().collect::<HashSet<_>>()
            );
        });
    }
}

#[cfg(test)]
mod tests_primitive_type_trait {
    use crate::{
        json_type::{EnumJsonType, JsonType},
        traits::_pyo3::perform_python_check,
    };
    use test_case::test_case;

    #[test_case("[]", EnumJsonType::Array)]
    #[test_case("True", EnumJsonType::Boolean)]
    #[test_case("1", EnumJsonType::Integer)]
    #[test_case("None", EnumJsonType::Null)]
    #[test_case("1.2", EnumJsonType::Number)]
    #[test_case("{'prop': 'value'}", EnumJsonType::Object)]
    #[test_case("'string'", EnumJsonType::String)]
    fn test_primitive_type(python_code_string: &str, expected_value: EnumJsonType) {
        perform_python_check(python_code_string, |python_object_ref| {
            assert_eq!(JsonType::primitive_type(python_object_ref), expected_value);
        })
    }

    #[test_case("{'present': 1}", "present", Some(1))]
    #[test_case("{'present': 1}", "not-present", None)]
    fn test_get_attribute(python_code_string: &str, attribute_name: &str, expected_value: Option<i128>) {
        perform_python_check(python_code_string, |python_object_ref| {
            assert_eq!(
                JsonType::get_attribute(python_object_ref, attribute_name).and_then(|value| value.as_integer()),
                expected_value
            );
        })
    }

    #[test_case("[0, 1, 2]", 1, Some(1))]
    #[test_case("[0, 1, 2]", 4, None)]
    fn test_get_index(python_code_string: &str, index: usize, expected_value: Option<i128>) {
        perform_python_check(python_code_string, |python_object_ref| {
            assert_eq!(JsonType::get_index(python_object_ref, index).and_then(|value| value.as_integer()), expected_value);
        })
    }

    #[test_case("{'present': 1}", "present", true)]
    #[test_case("{'present': 1}", "not-present", false)]
    #[test_case("[1, 2, 3]", "not-present", false)]
    fn test_has_attribute(python_code_string: &str, attr_name: &str, expected_value: bool) {
        perform_python_check(python_code_string, |python_object_ref| {
            assert_eq!(JsonType::has_attribute(python_object_ref, attr_name), expected_value);
        })
    }

    #[test_case("[0, 1, 2]", true)]
    #[test_case("True", false)]
    #[test_case("1", false)]
    #[test_case("None", false)]
    #[test_case("1.2", false)]
    #[test_case("{'key': 'value'}", false)]
    #[test_case("'string'", false)]
    fn test_is_array(python_code_string: &str, expected_value: bool) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::is_array(python_object_ref), expected_value))
    }

    #[test_case("[0, 1, 2]", false)]
    #[test_case("True", true)]
    #[test_case("1", false)]
    #[test_case("None", false)]
    #[test_case("1.2", false)]
    #[test_case("{'key': 'value'}", false)]
    #[test_case("'string'", false)]
    fn test_is_boolean(python_code_string: &str, expected_value: bool) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::is_boolean(python_object_ref), expected_value))
    }

    #[test_case("[0, 1, 2]", false)]
    #[test_case("True", false)]
    #[test_case("1", true)]
    #[test_case("None", false)]
    #[test_case("1.2", false)]
    #[test_case("{'key': 'value'}", false)]
    #[test_case("'string'", false)]
    fn test_is_integer(python_code_string: &str, expected_value: bool) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::is_integer(python_object_ref), expected_value))
    }

    #[test_case("[0, 1, 2]", false)]
    #[test_case("True", false)]
    #[test_case("1", false)]
    #[test_case("None", true)]
    #[test_case("1.2", false)]
    #[test_case("{'key': 'value'}", false)]
    #[test_case("'string'", false)]
    fn test_is_null(python_code_string: &str, expected_value: bool) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::is_null(python_object_ref), expected_value))
    }

    #[test_case("[0, 1, 2]", false)]
    #[test_case("True", false)]
    #[test_case("1", true)]
    #[test_case("None", false)]
    #[test_case("1.2", true)]
    #[test_case("{'key': 'value'}", false)]
    #[test_case("'string'", false)]
    fn test_is_number(python_code_string: &str, expected_value: bool) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::is_number(python_object_ref), expected_value))
    }

    #[test_case("[0, 1, 2]", false)]
    #[test_case("True", false)]
    #[test_case("1", false)]
    #[test_case("None", false)]
    #[test_case("1.2", false)]
    #[test_case("{'key': 'value'}", true)]
    #[test_case("'string'", false)]
    fn test_is_object(python_code_string: &str, expected_value: bool) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::is_object(python_object_ref), expected_value))
    }

    #[test_case("[0, 1, 2]", false)]
    #[test_case("True", false)]
    #[test_case("1", false)]
    #[test_case("None", false)]
    #[test_case("1.2", false)]
    #[test_case("{'key': 'value'}", false)]
    #[test_case("'string'", true)]
    fn test_is_string(python_code_string: &str, expected_value: bool) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::is_string(python_object_ref), expected_value))
    }

    #[test_case("[1]", true)]
    #[test_case("[1, 'a']", true)]
    #[test_case("None", false)]
    fn test_as_array(python_code_string: &str, is_some: bool) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::as_array(python_object_ref).is_some(), is_some))
    }

    #[test_case("True", Some(true))]
    #[test_case("False", Some(false))]
    #[test_case("1", None)]
    fn test_as_boolean(python_code_string: &str, expected_value: Option<bool>) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::as_boolean(python_object_ref), expected_value))
    }

    #[test_case("1", Some(1))]
    #[test_case("1.2", None)]
    #[test_case("'1'", None)]
    fn test_as_integer(python_code_string: &str, expected_value: Option<i128>) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::as_integer(python_object_ref), expected_value))
    }

    #[test_case("None", Some(()))]
    #[test_case("'1'", None)]
    fn test_as_null(python_code_string: &str, expected_value: Option<()>) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::as_null(python_object_ref), expected_value))
    }

    #[test_case("1", Some(1_f64))]
    #[test_case("1.2", Some(1.2))]
    #[test_case("'1'", None)]
    fn test_as_number(python_code_string: &str, expected_value: Option<f64>) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::as_number(python_object_ref), expected_value))
    }

    #[test_case("1", false)]
    #[test_case("1.2", false)]
    #[test_case("{'1': 1}", true)]
    fn test_as_object(python_code_string: &str, is_some: bool) {
        perform_python_check(python_code_string, |python_object_ref| {
            assert_eq!(JsonType::as_object(python_object_ref).is_some(), is_some);
        })
    }

    #[test_case("1", None)]
    #[test_case("1.2", None)]
    #[test_case("'1'", Some("1"))]
    fn test_as_string(python_code_string: &str, expected_value: Option<&str>) {
        perform_python_check(python_code_string, |python_object_ref| assert_eq!(JsonType::as_string(python_object_ref), expected_value))
    }
}

#[cfg(test)]
mod json_map_tests {
    use crate::{traits::_pyo3::perform_python_check, JsonMapTrait, JsonType};

    lazy_static! {
        static ref PYTHON_TESTING_MAP_STR: &'static str = "{'key1': {'key2': 1}}";
    }

    #[test]
    fn test_keys() {
        perform_python_check(&PYTHON_TESTING_MAP_STR, |python_object_ref| {
            let key1 = python_object_ref.get_attribute("key1").unwrap();
            assert_eq!(JsonType::as_object(key1).unwrap().keys().collect::<Vec<_>>(), vec![String::from("key2")]);
        });
    }

    #[test]
    fn test_values() {
        perform_python_check(&PYTHON_TESTING_MAP_STR, |python_object_ref| {
            let key1 = python_object_ref.get_attribute("key1").unwrap();
            assert_eq!(
                JsonType::as_object(key1).unwrap().values().map(|v| format!("{:?}", v)).collect::<Vec<_>>(),
                vec![String::from("1")],
            );
        });
    }

    #[test]
    fn test_items() {
        perform_python_check(&PYTHON_TESTING_MAP_STR, |python_object_ref| {
            let key1 = python_object_ref.get_attribute("key1").unwrap();
            assert_eq!(
                JsonType::as_object(key1).unwrap().items().map(|(k, v)| format!("{} -> {:?}", k, v)).collect::<Vec<_>>(),
                vec![String::from("key2 -> 1")],
            );
        });
    }
}
