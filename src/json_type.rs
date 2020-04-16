use crate::{error::Error, fragment::fragment_components_from_fragment, rust_type::RustType};
use std::{collections::HashMap, convert::TryFrom, fmt::Debug, ops::Deref};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, EnumIter, EnumVariantNames, Eq, Hash, Debug, Display, PartialEq)]
pub enum PrimitiveType {
    // We assume that all the drafts will have the same primitive types
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

impl TryFrom<&str> for PrimitiveType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "array" => Ok(Self::Array),
            "boolean" => Ok(Self::Boolean),
            "integer" => Ok(Self::Integer),
            "null" => Ok(Self::Null),
            "number" => Ok(Self::Number),
            "object" => Ok(Self::Object),
            "string" => Ok(Self::String),
            _ => Err(Error::UnsupportedPrimitiveType { type_str: value.to_string() }),
        }
    }
}

impl Into<&str> for PrimitiveType {
    fn into(self) -> &'static str {
        match self {
            Self::Array => "array",
            Self::Boolean => "boolean",
            Self::Integer => "integer",
            Self::Null => "null",
            Self::Number => "number",
            Self::Object => "object",
            Self::String => "string",
        }
    }
}

pub trait JsonMapTrait<'json, T: 'json + JsonType> {
    #[must_use]
    fn keys(&'json self) -> Box<dyn Iterator<Item = &str> + 'json>;

    #[must_use]
    fn values(&'json self) -> Box<dyn Iterator<Item = &T> + 'json>;

    #[must_use]
    fn items(&'json self) -> Box<dyn Iterator<Item = (&str, &T)> + 'json>;
}

pub trait ToRustType {
    fn to_rust_type(&self) -> RustType
    where
        Self: Sized + JsonType,
    {
        if let Some(array) = self.as_array() {
            RustType::from(array.map(|item| item.to_rust_type()).collect::<Vec<_>>())
        } else if let Some(bool) = self.as_boolean() {
            RustType::from(bool)
        } else if let Some(integer) = self.as_integer() {
            RustType::from(integer)
        } else if self.is_null() {
            RustType::from(())
        } else if let Some(number) = self.as_number() {
            RustType::from(number)
        } else if let Some(object) = self.as_object() {
            RustType::from(object.items().map(|(k, v)| (k.into(), v.to_rust_type())).collect::<HashMap<_, _>>())
        } else if let Some(string) = self.as_string() {
            RustType::from(string)
        } else {
            #[allow(unsafe_code)]
            unsafe {
                std::hint::unreachable_unchecked()
            }
        }
    }
}

// This trait allows us to have a 1:1 mapping with serde_json, generally used by rust libraries
// but gives us the power to use different objects from serde_json. This gives us the ability
// to support usage of different data-types like PyObject from pyo3 in case of python bindings
pub trait JsonType: Debug + ToRustType {
    fn as_array<'json>(&'json self) -> Option<Box<dyn ExactSizeIterator<Item = &Self> + 'json>>
    where
        Self: Sized;
    fn as_boolean(&self) -> Option<bool>;
    fn as_integer(&self) -> Option<i128>;
    fn as_null(&self) -> Option<()>;
    fn as_number(&self) -> Option<f64>;
    fn as_object(&self) -> Option<JsonMap<Self>>
    where
        Self: Sized;
    fn as_string(&self) -> Option<&str>;

    fn get_attribute(&self, attribute_name: &str) -> Option<&Self>
    where
        Self: Sized;
    fn get_index(&self, index: usize) -> Option<&Self>
    where
        Self: Sized;

    fn is_array(&self) -> bool
    where
        Self: Sized,
    {
        self.as_array().is_some()
    }

    fn is_boolean(&self) -> bool {
        self.as_boolean().is_some()
    }

    fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }

    fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    fn is_number(&self) -> bool {
        self.as_number().is_some()
    }

    fn is_object(&self) -> bool
    where
        Self: Sized,
    {
        self.as_object().is_some()
    }

    fn is_string(&self) -> bool {
        self.as_string().is_some()
    }

    fn has_attribute(&self, attribute_name: &str) -> bool
    where
        Self: Sized,
    {
        self.get_attribute(attribute_name).is_some()
    }

    fn primitive_type(&self) -> PrimitiveType
    where
        Self: Sized,
    {
        // This might not be efficient, but it could be comfortable to quickly extract the type especially while debugging
        if self.is_array() {
            PrimitiveType::Array
        } else if self.is_boolean() {
            PrimitiveType::Boolean
        } else if self.is_integer() {
            PrimitiveType::Integer
        } else if self.is_null() {
            PrimitiveType::Null
        } else if self.is_number() {
            PrimitiveType::Number
        } else if self.is_object() {
            PrimitiveType::Object
        } else if self.is_string() {
            PrimitiveType::String
        } else {
            #[allow(unsafe_code)]
            unsafe {
                std::hint::unreachable_unchecked()
            }
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait ThreadSafeJsonType: JsonType + Sync + Send {}

#[derive(Debug)]
pub struct JsonMap<'json, T: JsonType>(&'json T);

impl<'json, T: JsonType> JsonMap<'json, T> {
    pub fn new(object: &'json T) -> Self {
        Self(object)
    }
}

impl<'json, T: JsonType> Deref for JsonMap<'json, T> {
    type Target = T;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'json, T: JsonType> JsonMapTrait<'json, T> for JsonMap<'json, T> {
    #[must_use]
    default fn keys(&'json self) -> Box<dyn Iterator<Item = &str> + 'json> {
        Box::new(self.items().map(|(key, _)| key))
    }

    #[must_use]
    default fn values(&'json self) -> Box<dyn Iterator<Item = &T> + 'json> {
        Box::new(self.items().map(|(_, value)| value))
    }

    #[must_use]
    default fn items(&'json self) -> Box<dyn Iterator<Item = (&str, &T)>> {
        todo!("The library relies on specialization to reduce the amount of trait constraints needed for JsonType. That's why we have this dummy JsonMapTrait::items implementation. NOTE: All the types implementing JsonType trait should take care of implementing at least JsonMapTrait::items as well.")
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn get_fragment<'json, T: JsonType>(json_object: &'json T, fragment: &str) -> Option<&'json T> {
    let mut result = Some(json_object);
    for fragment_part in fragment_components_from_fragment(fragment) {
        if let Some(value) = result {
            result = match value.primitive_type() {
                PrimitiveType::Object => value.get_attribute(fragment_part.as_str()),
                PrimitiveType::Array => fragment_part.parse::<usize>().map(|index| value.get_index(index)).ok().unwrap_or(None),
                _ => None,
            };
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{get_fragment, Error, JsonType, PrimitiveType};
    use crate::rust_type::RustType;
    use std::convert::TryFrom;
    use test_case::test_case;

    #[test]
    fn test_ensure_that_jsontype_can_be_made_into_an_object() {
        // The code will fail to compile if JsonType cannot be made into an object
        // Adding `fn foo() {}` into the trait will result into
        // error[E0038]: the trait `json_type::JsonType` cannot be made into an object
        //     associated function `foo` has no `self` parameter
        fn check(_v: &dyn JsonType) {}
        check(&RustType::default())
    }

    #[test_case("array", &Ok(PrimitiveType::Array))]
    #[test_case("integer", &Ok(PrimitiveType::Integer))]
    #[test_case("number", &Ok(PrimitiveType::Number))]
    #[test_case("null", &Ok(PrimitiveType::Null))]
    #[test_case("object", &Ok(PrimitiveType::Object))]
    #[test_case("string", &Ok(PrimitiveType::String))]
    #[test_case("an invalid type", &Err(Error::UnsupportedPrimitiveType { type_str: "an invalid type".to_string() }))]
    fn test_enum_primitive_type_from_type(type_str: &str, expected_result: &Result<PrimitiveType, Error>) {
        assert_eq!(&PrimitiveType::try_from(type_str), expected_result);
    }

    #[test_case(PrimitiveType::Array, "array")]
    #[test_case(PrimitiveType::Integer, "integer")]
    #[test_case(PrimitiveType::Number, "number")]
    #[test_case(PrimitiveType::Null, "null")]
    #[test_case(PrimitiveType::Object, "object")]
    #[test_case(PrimitiveType::String, "string")]
    fn test_primitive_type_to_type(primitive_type: PrimitiveType, expected_type_str: &str) {
        let type_str: &str = primitive_type.into();
        assert_eq!(type_str, expected_type_str);
    }

    #[test]
    fn test_ensure_that_trait_can_be_made_into_an_object() {
        let _: Option<Box<dyn JsonType>> = None;
    }

    #[test_case("", &Some(rust_type_map!["key" => rust_type_map!["inner_key" => rust_type_vec![1, "2"]]]))]
    #[test_case("/key", &Some(rust_type_map!["inner_key" => rust_type_vec![1, "2"]]))]
    #[test_case("/key/inner_key", &Some(rust_type_vec![1,"2"]))]
    #[test_case("/key/inner_key/0", &Some(RustType::from(1)))]
    #[test_case("/key/inner_key/1", &Some(RustType::from("2")))]
    #[test_case("/not_present", &None)]
    #[test_case("/key/inner_key/a", &None)]
    #[test_case("/key/inner_key/2", &None)]
    fn test_get_fragment(fragment: &str, expected_value: &Option<RustType>) {
        let external_map = rust_type_map![
            "key" => rust_type_map![
                "inner_key" => rust_type_vec![
                    1,
                    "2"
                ],
            ],
        ];
        assert_eq!(get_fragment(&external_map, fragment), expected_value.as_ref());
    }
}
