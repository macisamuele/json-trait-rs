use crate::{fragment::fragment_components_from_fragment, index::Index};
use std::{fmt::Debug, ops::Deref};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, EnumIter, Eq, Debug, Display, PartialEq)]
pub enum EnumJsonType {
    // We assume that all the drafts will have the same primitive types
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

impl EnumJsonType {
    pub fn from_type(type_string: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match type_string {
            "array" => Some(EnumJsonType::Array),
            "boolean" => Some(EnumJsonType::Boolean),
            "integer" => Some(EnumJsonType::Integer),
            "null" => Some(EnumJsonType::Null),
            "number" => Some(EnumJsonType::Number),
            "object" => Some(EnumJsonType::Object),
            "string" => Some(EnumJsonType::String),
            _ => None,
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            EnumJsonType::Array => "array",
            EnumJsonType::Boolean => "boolean",
            EnumJsonType::Integer => "integer",
            EnumJsonType::Null => "null",
            EnumJsonType::Number => "number",
            EnumJsonType::Object => "object",
            EnumJsonType::String => "string",
        }
    }
}

pub trait JsonMapTrait<'json, T>
where
    T: 'json + JsonType<'json>,
    JsonMap<'json, T>: JsonMapTrait<'json, T>,
{
    #[inline]
    fn keys(&'json self) -> Box<ExactSizeIterator<Item = &str> + 'json> {
        Box::new(self.items().map(|(key, _)| key))
    }

    #[inline]
    fn values(&'json self) -> Box<ExactSizeIterator<Item = &T> + 'json> {
        Box::new(self.items().map(|(_, value)| value))
    }

    fn items(&'json self) -> Box<ExactSizeIterator<Item = (&str, &T)> + 'json>;
}

// This trait allows us to have a 1:1 mapping with serde_json, generally used by rust libraries
// but gives us the power to use different objects from serde_json. This gives us the ability
// to support usage of different data-types like PyObject from pyo3 in case of python bindings
pub trait JsonType<'json>: 'json + Clone + Debug + PartialEq + Sync + Send
where
    JsonMap<'json, Self>: JsonMapTrait<'json, Self>,
{
    fn as_array(&'json self) -> Option<Box<ExactSizeIterator<Item = &Self> + 'json>>;
    fn as_boolean(&'json self) -> Option<bool>;
    fn as_integer(&'json self) -> Option<i128>;
    fn as_null(&'json self) -> Option<()>;
    fn as_number(&'json self) -> Option<f64>;
    fn as_object(&'json self) -> Option<JsonMap<'json, Self>>;
    fn as_string(&'json self) -> Option<&str>;

    fn get_attribute<R: AsRef<str>>(&'json self, attribute_name: R) -> Option<&Self>;
    fn get_index(&'json self, index: usize) -> Option<&Self>;

    #[inline]
    fn is_array(&'json self) -> bool {
        self.as_array().is_some()
    }

    #[inline]
    fn is_boolean(&'json self) -> bool {
        self.as_boolean().is_some()
    }

    #[inline]
    fn is_integer(&'json self) -> bool {
        self.as_integer().is_some()
    }

    #[inline]
    fn is_null(&'json self) -> bool {
        self.as_null().is_some()
    }

    #[inline]
    fn is_number(&'json self) -> bool {
        self.as_number().is_some()
    }

    #[inline]
    fn is_object(&'json self) -> bool {
        self.as_object().is_some()
    }

    #[inline]
    fn is_string(&'json self) -> bool {
        self.as_string().is_some()
    }

    #[inline]
    fn has_attribute(&'json self, attribute_name: &str) -> bool {
        self.get_attribute(attribute_name).is_some()
    }

    #[inline]
    fn get<I: Index<'json, Self>>(&'json self, index: I) -> Option<&Self> {
        index.index_into(self)
    }

    #[inline]
    fn primitive_type(&'json self) -> EnumJsonType {
        // This might not be efficient, but it could be comfortable to quickly extract the type especially while debugging
        if self.is_array() {
            EnumJsonType::Array
        } else if self.is_boolean() {
            EnumJsonType::Boolean
        } else if self.is_integer() {
            EnumJsonType::Integer
        } else if self.is_null() {
            EnumJsonType::Null
        } else if self.is_number() {
            EnumJsonType::Number
        } else if self.is_object() {
            EnumJsonType::Object
        } else if self.is_string() {
            EnumJsonType::String
        } else {
            #[allow(unsafe_code)]
            unsafe {
                unreachable::unreachable()
            }
        }
    }

    fn fragment<R: AsRef<str>>(&'json self, fragment: R) -> Option<&Self> {
        // NOTE: Iteration order matters, so iterate![] should not be used here
        fragment_components_from_fragment(fragment.as_ref())
            // Using fold as for now tail recursion is not supported in rust, but if it will ever happen then `fold` will most probably be the first candidate
            .fold(Some(self), |result, fragment_part| {
                result.and_then(|value| match value.primitive_type() {
                    EnumJsonType::Object => value.get_attribute(fragment_part),
                    EnumJsonType::Array => fragment_part
                        .parse::<usize>()
                        .and_then(|index| Ok(value.get_index(index)))
                        .ok()
                        .unwrap_or(None),
                    _ => None,
                })
            })
    }
}

#[derive(Debug)]
pub struct JsonMap<'json, T>(&'json T)
where
    T: 'json + JsonType<'json>,
    JsonMap<'json, T>: JsonMapTrait<'json, T>;

impl<'json, T> JsonMap<'json, T>
where
    T: 'json + JsonType<'json>,
    JsonMap<'json, T>: JsonMapTrait<'json, T>,
{
    #[inline]
    pub fn new(object: &'json T) -> Self {
        Self(object)
    }
}

impl<'json, T> Deref for JsonMap<'json, T>
where
    T: 'json + JsonType<'json>,
    JsonMap<'json, T>: JsonMapTrait<'json, T>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[cfg(test)]
mod enum_primitive_type_tests {
    use super::EnumJsonType;
    use test_case_derive::test_case;

    #[test_case("array", Some(EnumJsonType::Array))]
    #[test_case("integer", Some(EnumJsonType::Integer))]
    #[test_case("number", Some(EnumJsonType::Number))]
    #[test_case("null", Some(EnumJsonType::Null))]
    #[test_case("object", Some(EnumJsonType::Object))]
    #[test_case("string", Some(EnumJsonType::String))]
    #[test_case("an invalid type", None)]
    fn from_type(type_str: &str, expected_option_enum_primitive_type: Option<EnumJsonType>) {
        assert_eq!(EnumJsonType::from_type(type_str), expected_option_enum_primitive_type);
    }

    #[test_case(EnumJsonType::Array, "array")]
    #[test_case(EnumJsonType::Integer, "integer")]
    #[test_case(EnumJsonType::Number, "number")]
    #[test_case(EnumJsonType::Null, "null")]
    #[test_case(EnumJsonType::Object, "object")]
    #[test_case(EnumJsonType::String, "string")]
    fn to_type(enum_primitive_type: EnumJsonType, expected_type_str: &str) {
        assert_eq!(enum_primitive_type.to_type(), expected_type_str);
    }
}

#[cfg(test)]
mod primitive_type_tests {
    #[allow(unused_imports)]
    use crate::json_type::JsonType;
    use crate::testing::TestingType;
    use test_case_derive::test_case;

    #[test_case("", Some(&testing_map![
        "key" => testing_map![
            "inner_key" => testing_vec![
                1,
                "2"
            ],
        ],
    ]))]
    #[test_case("/key", Some(&testing_map![
        "inner_key" => testing_vec![
            1,
            "2"
        ],
    ]))]
    #[test_case("/key/inner_key", Some(&testing_vec![
        1,
        "2"
    ]))]
    #[test_case("/key/inner_key/0", Some(&TestingType::from(1)))]
    #[test_case("/key/inner_key/1", Some(&TestingType::from("2")))]
    #[test_case("/not_present", None)]
    #[test_case("/key/inner_key/a", None)]
    #[test_case("/key/inner_key/2", None)]
    fn test_get_fragment(fragment: &str, expected_value: Option<&TestingType>) {
        let external_map = testing_map![
            "key" => testing_map![
                "inner_key" => testing_vec![
                    1,
                    "2"
                ],
            ],
        ];
        assert_eq!(external_map.fragment(fragment), expected_value);
    }
}
