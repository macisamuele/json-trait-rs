use failure::Fail;
#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "Unsupported primitive type `{}`. Available types are defined by `json_trait_rs::PrimitiveType::VARIANTS`", type_str)]
    UnsupportedPrimitiveType { type_str: String },
}
