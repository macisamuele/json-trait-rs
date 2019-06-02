Changelog
=========

0.3.0 (2019-06-02)
------------------
- Export macros and rename ``TestingType`` into ``RustType`` - [PR #3](https://github.com/macisamuele/json-trait-rs/pull/3)

0.2.0 (2019-05-26)
------------------
- Consider private internal modules and expose, in the top level, the important structs, enum and traits - [PR #1](https://github.com/macisamuele/json-trait-rs/pull/1/)
- Simplify JsonType signature (remove lifetime and JsonMap constraint from the class) - [PR #1](https://github.com/macisamuele/json-trait-rs/pull/1/)

0.1.0 (2019-04-25)
------------------
- Initial project release
- Definition of `JsonType` trait
- Implementation of trait for [`json::JsonValue`](https://github.com/maciejhirsz/json-rust/), [`serde_json::Value`](https://github.com/serde-rs/json/) and [`serde_yaml::Value`](https://github.com/dtolnay/serde-yaml).
