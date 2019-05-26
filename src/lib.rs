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
// Ignore missing_const_for_fn clippy linter (it's too noisy in regards const fn in traits)
#![allow(clippy::missing_const_for_fn)]

#[macro_use]
extern crate strum_macros;
#[cfg(all(test, any(feature = "trait_serde_json", feature = "trait_serde_yaml", feature = "trait_json")))]
#[macro_use]
extern crate lazy_static;
#[cfg(all(test, any(feature = "trait_serde_json", feature = "trait_serde_yaml", feature = "trait_json")))]
#[cfg(test)]
#[macro_use]
extern crate serde_json;

// Macros have to be imported first to allow usage on other modules
#[cfg(test)]
#[macro_use]
mod macros;

mod fragment;
mod index;
pub mod json_type;
pub mod testing;
pub mod traits;
