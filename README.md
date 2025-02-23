# py-rs

<h1 align="center" style="padding-top: 0; margin-top: 0;">
<img width="150px" src="https://raw.githubusercontent.com/Blue-Chestnut/py-rs/main/logo.png" alt="logo">
<br/>
py-rs
</h1>
<p align="center">
Generate python type declarations from rust types
</p>

<div align="center">
<!-- Github Actions -->
<img src="https://img.shields.io/github/actions/workflow/status/Blue-Chestnut/py-rs/test.yml?branch=main" alt="actions status" />
<a href="https://crates.io/crates/py-rs">
<img src="https://img.shields.io/crates/v/py-rs.svg?style=flat-square"
alt="Crates.io version" />
</a>
<a href="https://docs.rs/py-rs">
<img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
alt="docs.rs docs" />
</a>
<a href="https://crates.io/crates/py-rs">
<img src="https://img.shields.io/crates/d/py-rs.svg?style=flat-square"
alt="Download" />
</a>
</div>

### Why?
When building an api in rust, data structures have to be shared between backend
and client. Using this library, you can easily generate python bindings to your
rust structs & enums so that you can keep your types in one place.

> **Note:** This is a work in progress. There are still some features of `ts-rs` that are not tested or converted.

### How?
py-rs exposes a single trait, `PY`. Using a derive macro, you can implement this interface for your types.
Then, you can use this trait to obtain the Python bindings.
We recommend doing this in your tests.
[See the example](https://github.com/Blue-Chestnut/py-rs/blob/main/example/src/lib.rs) and [the docs](https://docs.rs/py-rs/latest/py_rs/).

### Get started
```toml
[dependencies]
py-rs = "0.1.0"
```

```rust
use py_rs::PY;

#[derive(PY)]
#[py(export)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
}
```

When running `cargo test` or `cargo test export_bindings`, the Python bindings will be exported to the file `bindings/User.py`
and will contain the following code:

```python
from pydanic import BaseModel

class User(BaseModel):
    user_id: int
    first_name: str
    last_name: str
```

### Features
- generate type declarations from rust structs
- generate union declarations from rust enums
- generate necessary imports when exporting to multiple files
- serde compatibility
- generic types
- support for ESM imports

> **Note:** not all the features are tested for Python.

### cargo features
| **Feature**        | **Description**                                                                                                                                                                                           |
|:-------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| serde-compat       | **Enabled by default** <br/>See the *"serde compatibility"* section below for more information.                                                                                                           |
| format             | Enables formatting of the generated Python bindings. <br/>Currently, this unfortunately adds quite a few dependencies.                                                                                    |
| no-serde-warnings  | By default, warnings are printed during build if unsupported serde attributes are encountered. <br/>Enabling this feature silences these warnings.                                                        |
| serde-json-impl    | Implement `PY` for types from *serde_json*                                                                                                                                                                |
| chrono-impl        | Implement `PY` for types from *chrono*                                                                                                                                                                    |
| bigdecimal-impl    | Implement `PY` for types from *bigdecimal*                                                                                                                                                                |
| url-impl           | Implement `PY` for types from *url*                                                                                                                                                                       |
| uuid-impl          | Implement `PY` for types from *uuid*                                                                                                                                                                      |
| bson-uuid-impl     | Implement `PY` for *bson::oid::ObjectId* and *bson::uuid*                                                                                                                                                 |
| bytes-impl         | Implement `PY` for types from *bytes*                                                                                                                                                                     |
| indexmap-impl      | Implement `PY` for types from *indexmap*                                                                                                                                                                  |
| ordered-float-impl | Implement `PY` for types from *ordered_float*                                                                                                                                                             |
| heapless-impl      | Implement `PY` for types from *heapless*                                                                                                                                                                  |
| semver-impl        | Implement `PY` for types from *semver*                                                                                                                                                                    |
| smol_str-impl      | Implement `PY` for types from *smol_str*                                                                                                                                                                  |
| tokio-impl         | Implement `PY` for types from *tokio*                                                                                                                                                                     |

<br/>

If there's a type you're dealing with which doesn't implement `PY`, use either
`#[py(as = "..")]` or `#[py(type = "..")]`, or open a PR.

### `serde` compatability
With the `serde-compat` feature (enabled by default), serde attributes can be parsed for enums and structs.
Supported serde attributes:
- `rename`
- `rename-all`
- `rename-all-fields`
- `tag`
- `content`
- `untagged`
- `skip`
- `flatten`
- `default`

Note: `skip_serializing` and `skip_deserializing` are ignored. If you wish to exclude a field
from the generated type, but cannot use `#[serde(skip)]`, use `#[py(skip)]` instead.

When py-rs encounters an unsupported serde attribute, a warning is emitted, unless the feature `no-serde-warnings` is enabled.

### Contributing
Contributions are always welcome!
Feel free to open an issue, discuss using GitHub discussions or open a PR.
[See CONTRIBUTING.md](https://github.com/Blue-Chestnut/py-rs/blob/main/CONTRIBUTING.md)

### MSRV
The Minimum Supported Rust Version for this crate is 1.63.0

License: MIT
