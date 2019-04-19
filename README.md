# json-trait-rs

[![Linux Build on TravisCI](https://img.shields.io/travis/com/macisamuele/json-trait-rs/master.svg?logo=travis&label=Linux)](https://travis-ci.com/macisamuele/json-trait-rs)
[![Coverage](https://img.shields.io/codecov/c/github/macisamuele/json-trait-rs/master.svg)](https://codecov.io/gh/macisamuele/json-trait-rs)

[Changelog](./CHANGELOG.md)

## Rationale
The goal of this repository is to offer rust interfaces (aka traits) to deal with objects as they are JSON objects.

While dealing with JSON objects in rust we tend to use libraries that takes care of the serialisation and desrialisation
process as [serde-json](https://github.com/serde-rs/json) or [json-rust](https://github.com/maciejhirsz/json-rust), but
depending on the use-case it's possible that we do need only methodologies to traverse the objects that have been
created (so [de]serialised) by other libraries.

The main use-case for this type of library, at least at the time of writing, is to offer JSON objects traversing
capability to objects that might be initialised by foreign languages (think to other language bindings, ie. via
[FFI](https://en.wikipedia.org/wiki/Foreign_function_interface)).

## Contribution rules
Coming soon
