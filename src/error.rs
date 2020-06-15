use thiserror::Error;
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Unsupported primitive type `{type_str}`. Available types are defined by `json_trait_rs::PrimitiveType::VARIANTS`")]
    UnsupportedPrimitiveType { type_str: String },
}
