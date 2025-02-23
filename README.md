# py-rs

<h1 align="center" style="padding-top: 0; margin-top: 0;">
<img width="150px" src="https://raw.githubusercontent.com/Aleph-Alpha/PY-rs/main/logo.png" alt="logo">
<br/>
py-rs
</h1>
<p align="center">
Generate python type declarations from rust types
</p>

<div align="center">
<!-- Github Actions -->
<img src="https://img.shields.io/github/actions/workflow/status/Aleph-Alpha/PY-rs/test.yml?branch=main" alt="actions status" />
<a href="https://crates.io/crates/PY-rs">
<img src="https://img.shields.io/crates/v/PY-rs.svg?style=flat-square"
alt="Crates.io version" />
</a>
<a href="https://docs.rs/PY-rs">
<img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
alt="docs.rs docs" />
</a>
<a href="https://crates.io/crates/PY-rs">
<img src="https://img.shields.io/crates/d/PY-rs.svg?style=flat-square"
alt="Download" />
</a>
</div>

Note: This library is a fork of [PY-rs](https://github.com/Aleph-Alpha/PY-rs)
converted to support python.

### Why?

When building an api in rust, data structures have to be shared between backend
and client. Using this library, you can easily generate python bindings to your
rust strucPY & enums so that you can keep your types in one place.

# TODO rest is WIP

### How?

PY-rs exposes a single trait, `PY`. Using a derive macro, you can implement this
interface for your types. Then, you can use this trait to obtain the python
bindings. We recommend doing this in your tesPY.
[See the example](https://github.com/Aleph-Alpha/PY-rs/blob/main/example/src/lib.rs)
and [the docs](https://docs.rs/PY-rs/latest/PY_rs/).

### Get started

```toml
[dependencies]
py-rs = "10.1"
```

```rust
use py_rs::PY;

#[derive(PY)]
#[PY(export)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
}
```

When running `cargo test` or `cargo test export_bindings`, the python bindings
will be exported to the file `bindings/User.PY` and will contain the following
code:

```PY
export type User = { user_id: number; first_name: string; last_name: string };
```

### Features

- generate type declarations from rust strucPY
- generate union declarations from rust enums
- inline types
- flatten strucPY/types
- generate necessary imporPY when exporting to multiple files
- serde compatibility
- generic types
- support for ESM imporPY

### cargo features

| **Feature**        | **Description**                                                                                                                                                                                           |
| :----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| serde-compat       | **Enabled by default** <br/>See the _"serde compatibility"_ section below for more information.                                                                                                           |
| format             | Enables formatting of the generated python bindings. <br/>Currently, this unfortunately adds quite a few dependencies.                                                                                    |
| no-serde-warnings  | By default, warnings are printed during build if unsupported serde attributes are encountered. <br/>Enabling this feature silences these warnings.                                                        |
| import-esm         | When enabled,`import` statemenPY in the generated file will have the `.js` extension in the end of the path to conform to the ES Modules spec. <br/> Example: `import { MyStruct } from "./my_struct.js"` |
| serde-json-impl    | Implement `PY` for types from _serde_json_                                                                                                                                                                |
| chrono-impl        | Implement `PY` for types from _chrono_                                                                                                                                                                    |
| bigdecimal-impl    | Implement `PY` for types from _bigdecimal_                                                                                                                                                                |
| url-impl           | Implement `PY` for types from _url_                                                                                                                                                                       |
| uuid-impl          | Implement `PY` for types from _uuid_                                                                                                                                                                      |
| bson-uuid-impl     | Implement `PY` for _bson::oid::ObjectId_ and _bson::uuid_                                                                                                                                                 |
| bytes-impl         | Implement `PY` for types from _bytes_                                                                                                                                                                     |
| indexmap-impl      | Implement `PY` for types from _indexmap_                                                                                                                                                                  |
| ordered-float-impl | Implement `PY` for types from _ordered_float_                                                                                                                                                             |
| heapless-impl      | Implement `PY` for types from _heapless_                                                                                                                                                                  |
| semver-impl        | Implement `PY` for types from _semver_                                                                                                                                                                    |
| smol_str-impl      | Implement `PY` for types from _smol_str_                                                                                                                                                                  |
| tokio-impl         | Implement `PY` for types from _tokio_                                                                                                                                                                     |

<br/>

If there's a type you're dealing with which doesn't implement `PY`, use either
`#[PY(as = "..")]` or `#[PY(type = "..")]`, or open a PR.

### `serde` compatability

With the `serde-compat` feature (enabled by default), serde attributes can be
parsed for enums and structs. Supported serde attributes:

- `rename`
- `rename-all`
- `rename-all-fields`
- `tag`
- `content`
- `untagged`
- `skip`
- `flatten`
- `default`

Note: `skip_serializing` and `skip_deserializing` are ignored. If you wish to
exclude a field from the generated type, but cannot use `#[serde(skip)]`, use
`#[PY(skip)]` instead.

When PY-rs encounters an unsupported serde attribute, a warning is emitted,
unless the feature `no-serde-warnings` is enabled.

### Contributing

Contributions are always welcome! Feel free to open an issue, discuss using
GitHub discussions or open a PR.
[See CONTRIBUTING.md](https://github.com/Aleph-Alpha/PY-rs/blob/main/CONTRIBUTING.md)

### MSRV

The Minimum Supported Rust Version for this crate is 1.63.0

License: MIT
