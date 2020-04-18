Changelog
=========

0.10.0 (2020-04-18)
-------------------
- Use specialization to have default implementation for JsonMap such tht trait constraints would not be required - [PR #33][https://github.com/macisamuele/json-trait-rs/pull/33)

0.9.0 (2020-04-12)
------------------
- Update `json_trait_rs::json_type::JsonType` trait to remove generic type from trait and to update trait hierarchy to shorten trait usage - [PR #31](https://github.com/macisamuele/json-trait-rs/pull/31)

WARNING: This reduces the Into<RustType> guarantee (even if we manually guarantee it) and ToRustType does still provide access point to a similar feature

0.8.0 (2020-03-23)
------------------
- Implement `json_trait_rs::JsonType::to_rust_type` (similar to `Into<RustType>` but from a reference)

0.7.0 (2020-03-04)
------------------
- `json_trait_rs::JsonType::as_array` returns `ExactSizeIterator` - [PR #24](https://github.com/macisamuele/json-trait-rs/pull/24)
- Improve `json_trait_rs::RustType` definition to store floats as well - [PR #26](https://github.com/macisamuele/json-trait-rs/pull/26)
- `json_trait_rs::JsonType` must implement `Into<json_trait_rs::RustType>` - [PR #26](https://github.com/macisamuele/json-trait-rs/pull/26)
- Rename `json_trait_rs::EnumJsonType` into `json_trait_rs::PrimitiveType` and implement `TryFrom<&str>`, `Into<&str>`, `Hash` and `Debug` - [PR #26](https://github.com/macisamuele/json-trait-rs/pull/26)
- Define `json_trait_rs::Error` to collect all the errors that the crate might report (based on `failure` crate) - [PR #26](https://github.com/macisamuele/json-trait-rs/pull/26)

0.6.0 (2019-12-19)
------------------
- Relax ``JsonType`` to not be thread safe and define ``ThreadSafeJsonType`` to provide the thread safe version - [PR #22](https://github.com/macisamuele/json-trait-rs/pull/22)

0.5.1 (2019-10-19)
------------------
- Reduce usage of ``#[inline]`` and ensure that method results are used via ``#[must_use]`` - [PR #17](https://github.com/macisamuele/json-trait-rs/pull/17)

0.5.0 (2019-09-29)
------------------
- Replace [`test-crate-derive`](https://github.com/synek317/test-case-derive/) with [`test-crate`](https://github.com/frondeus/test-case) - [PR #14](https://github.com/macisamuele/json-trait-rs/pull/14)<br/>
  There are no API and/or feature changes respect previous changes, but as non-test code was modified I've releaesed a new version ;)

0.4.0 (2019-07-28)
------------------
- Ensure that `json_trait_rs::JsonType` traits can be made into object (ie. `Box<dyn JsonType<_>>`) - [PR #6](https://github.com/macisamuele/json-trait-rs/pull/6)<br/>
  **WARNING** this required:
  - change in the `json_trait_rs::JsonType` definition as now needs `Self` to be specified (`JsonType<Self>`)
  - removal of `json_trait_rs::JsonType::get` method to extract array item or attribute value
  - removal of `json_trait_rs::JsonType::fragment`.<br/>
    The functionality has been migrated into `json_trait_rs::get_fragment`
- Implement `json_trait_rs::JsonType` trait for [`pyo3::types::PyAny`](https://github.com/PyO3/pyo3) - [PR #9](https://github.com/macisamuele/json-trait-rs/pull/9)
- Simplify lifetimes across the code-base - [PR #7](https://github.com/macisamuele/json-trait-rs/pull/7) and [PR #8](https://github.com/macisamuele/json-trait-rs/pull/8)
- Cleanup `json_trait_rs::JsonMapTrait` implementation for `json::JsonValue` (`trait_json` feature) - [PR #11](https://github.com/macisamuele/json-trait-rs/pull/11)
- Rename macros: `testing_map` into `rust_type_map` and `testing_vec` into `rust_type_vec` -  [PR #5](https://github.com/macisamuele/json-trait-rs/pull/5)
- Enable tests execution on rust stable, simplify coverage instrumentation and run on OS X and Windows - [PR #4](https://github.com/macisamuele/json-trait-rs/pull/4), [PR #10](https://github.com/macisamuele/json-trait-rs/pull/10) and [PR #13](https://github.com/macisamuele/json-trait-rs/pull/13)

0.3.0 (2019-06-02)
------------------
- Export macros and rename ``json_trait_rs::TestingType`` into ``json_trait_rs::RustType`` - [PR #3](https://github.com/macisamuele/json-trait-rs/pull/3)

0.2.0 (2019-05-26)
------------------
- Consider private internal modules and expose, in the top level, the important structs, enum and traits - [PR #1](https://github.com/macisamuele/json-trait-rs/pull/1/)
- Simplify `json_trait_rs::JsonType` signature (remove lifetime and `json_trait_rs::JsonMap` constraint from the class) - [PR #1](https://github.com/macisamuele/json-trait-rs/pull/1/)

0.1.0 (2019-04-25)
------------------
- Initial project release
- Definition of `JsonType` trait
- Implementation of trait for [`json::JsonValue`](https://github.com/maciejhirsz/json-rust/), [`serde_json::Value`](https://github.com/serde-rs/json/) and [`serde_yaml::Value`](https://github.com/dtolnay/serde-yaml).
