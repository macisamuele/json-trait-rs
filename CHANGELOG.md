Changelog
=========

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
