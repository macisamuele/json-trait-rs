#![deny(
    unreachable_pub,
    anonymous_parameters,
    bad_style,
    const_err,
    dead_code,
    deprecated,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    late_bound_lifetime_arguments,
    missing_copy_implementations,
    missing_debug_implementations,
    // missing_docs,
    non_shorthand_field_patterns,
    non_upper_case_globals,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unreachable_code,
    unreachable_patterns,
    unsafe_code,
    unused_allocation,
    unused_assignments,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_extern_crates,
    unused_import_braces,
    unused_imports,
    unused_macros,
    unused_parens,
    unused_qualifications,
    unused_results,
    unused_unsafe,
    unused_variables,
    warnings,
)]
// Enable very pendantic clippy linting
#![deny(clippy::pedantic, clippy::nursery)]
// Used by json_type_rs::json_type::JsonMapTrait default implementation by json_type_rs::json_type::JsonMap
// This is mostly needed to reduce the amount of trait constraints when using json_type_rs::json_type::JsonType
#![feature(specialization)]

#[macro_use]
extern crate strum_macros;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(all(test, any(feature = "trait_serde_json", feature = "trait_serde_yaml", feature = "trait_json")))]
#[macro_use]
extern crate serde_json;

// Macros have to be imported first to allow usage on other modules
#[macro_use]
pub mod macros;

mod error;
mod fragment;
mod json_type;
mod rust_type;
pub mod traits;

pub use crate::{
    error::Error,
    json_type::{get_fragment, JsonMap, JsonMapTrait, JsonType, PrimitiveType, ThreadSafeJsonType, ToRustType},
    rust_type::RustType,
};
