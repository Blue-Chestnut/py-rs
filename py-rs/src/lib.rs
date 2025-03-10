//! <h1 align="center" style="padding-top: 0; margin-top: 0;">
//! <img width="150px" src="https://raw.githubusercontent.com/Blue-Chestnut/py-rs/main/logo.png" alt="logo">
//! <br/>
//! py-rs
//! </h1>
//! <p align="center">
//! Generate python type declarations from rust types
//! </p>
//!
//! <div align="center">
//! <!-- Github Actions -->
//! <img src="https://img.shields.io/github/actions/workflow/status/Blue-Chestnut/py-rs/test.yml?branch=main" alt="actions status" />
//! <a href="https://crates.io/crates/py-rs">
//! <img src="https://img.shields.io/crates/v/py-rs.svg?style=flat-square"
//! alt="Crates.io version" />
//! </a>
//! <a href="https://docs.rs/py-rs">
//! <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//! alt="docs.rs docs" />
//! </a>
//! <a href="https://crates.io/crates/py-rs">
//! <img src="https://img.shields.io/crates/d/py-rs.svg?style=flat-square"
//! alt="Download" />
//! </a>
//! </div>
//!
//! ## Why?
//! When building an api in rust, data structures have to be shared between backend
//! and client. Using this library, you can easily generate python bindings to your
//! rust structs & enums so that you can keep your types in one place.
//!
//! > **Note:** This is a work in progress. There are still some features of `ts-rs` that are not tested or converted.
//!
//! ## How?
//! py-rs exposes a single trait, `PY`. Using a derive macro, you can implement this interface for your types.
//! Then, you can use this trait to obtain the Python bindings.
//! We recommend doing this in your tests.
//! [See the example](https://github.com/Blue-Chestnut/py-rs/blob/main/example/src/lib.rs) and [the docs](https://docs.rs/py-rs/latest/py_rs/).
//!
//! ## Get started
//! ```toml
//! [dependencies]
//! py-rs = "0.1.0"
//! ```
//!
//! ```rust
//! use py_rs::PY;
//!
//! #[derive(PY)]
//! #[py(export)]
//! struct User {
//!     user_id: i32,
//!     first_name: String,
//!     last_name: String,
//! }
//! ```
//!
//! When running `cargo test` or `cargo test export_bindings`, the Python bindings will be exported to the file `bindings/User.py`
//! and will contain the following code:
//!
//! ```python
//! from pydanic import BaseModel
//!
//! class User(BaseModel):
//!     user_id: int
//!     first_name: str
//!     last_name: str
//! ```
//!
//! ## Features
//! - generate type declarations from rust structs
//! - generate union declarations from rust enums
//! - generate necessary imports when exporting to multiple files
//! - serde compatibility
//! - generic types
//! - support for ESM imports
//!
//! > **Note:** not all the features are tested for Python.
//!
//! ## cargo features
//! | **Feature**        | **Description**                                                                                                                                                                                           |
//! |:-------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | serde-compat       | **Enabled by default** <br/>See the *"serde compatibility"* section below for more information.                                                                                                           |
//! | format             | Enables formatting of the generated Python bindings. <br/>Currently, this unfortunately adds quite a few dependencies.                                                                                    |
//! | no-serde-warnings  | By default, warnings are printed during build if unsupported serde attributes are encountered. <br/>Enabling this feature silences these warnings.                                                        |
//! | serde-json-impl    | Implement `PY` for types from *serde_json*                                                                                                                                                                |
//! | chrono-impl        | Implement `PY` for types from *chrono*                                                                                                                                                                    |
//! | bigdecimal-impl    | Implement `PY` for types from *bigdecimal*                                                                                                                                                                |
//! | url-impl           | Implement `PY` for types from *url*                                                                                                                                                                       |
//! | uuid-impl          | Implement `PY` for types from *uuid*                                                                                                                                                                      |
//! | bson-uuid-impl     | Implement `PY` for *bson::oid::ObjectId* and *bson::uuid*                                                                                                                                                 |
//! | bytes-impl         | Implement `PY` for types from *bytes*                                                                                                                                                                     |
//! | indexmap-impl      | Implement `PY` for types from *indexmap*                                                                                                                                                                  |
//! | ordered-float-impl | Implement `PY` for types from *ordered_float*                                                                                                                                                             |
//! | heapless-impl      | Implement `PY` for types from *heapless*                                                                                                                                                                  |
//! | semver-impl        | Implement `PY` for types from *semver*                                                                                                                                                                    |
//! | smol_str-impl      | Implement `PY` for types from *smol_str*                                                                                                                                                                  |
//! | tokio-impl         | Implement `PY` for types from *tokio*                                                                                                                                                                     |
//!
//! <br/>
//!
//! If there's a type you're dealing with which doesn't implement `PY`, use either
//! `#[py(as = "..")]` or `#[py(type = "..")]`, or open a PR.
//!
//! ## `serde` compatability
//! With the `serde-compat` feature (enabled by default), serde attributes can be parsed for enums and structs.
//! Supported serde attributes:
//! - `rename`
//! - `rename-all`
//! - `rename-all-fields`
//! - `tag`
//! - `content`
//! - `untagged`
//! - `skip`
//! - `flatten`
//! - `default`
//!
//! Note: `skip_serializing` and `skip_deserializing` are ignored. If you wish to exclude a field
//! from the generated type, but cannot use `#[serde(skip)]`, use `#[py(skip)]` instead.
//!
//! When py-rs encounters an unsupported serde attribute, a warning is emitted, unless the feature `no-serde-warnings` is enabled.
//!
//! ## Contributing
//! Contributions are always welcome!
//! Feel free to open an issue, discuss using GitHub discussions or open a PR.
//! [See CONTRIBUTING.md](https://github.com/Blue-Chestnut/py-rs/blob/main/CONTRIBUTING.md)
//!
//! ## MSRV
//! The Minimum Supported Rust Version for this crate is 1.63.0

use std::{
    any::TypeId,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Range, RangeInclusive},
    path::{Path, PathBuf},
};

pub use py_rs_macros::PY;

pub use crate::export::ExportError;

#[cfg(feature = "chrono-impl")]
mod chrono;
mod export;
#[cfg(feature = "serde-json-impl")]
mod serde_json;
#[cfg(feature = "tokio-impl")]
mod tokio;

/// A type which can be represented in Python.  
/// Most of the time, you'd want to derive this trait instead of implementing it manually.  
/// py-rs comes with implementations for all primitives, most collections, tuples,
/// arrays and containers.
///
/// ### exporting
/// Because Rusts procedural macros are evaluated before other compilation steps, Python
/// bindings __cannot__ be exported during compile time.
///
/// Bindings can be exported within a test, which py-rs generates for you by adding `#[py(export)]`
/// to a type you wish to export to a file.  
/// When `cargo test` is run, all types annotated with `#[py(export)]` and all of their
/// dependencies will be written to `PY_RS_EXPORT_DIR`, or `./bindings` by default.
///
/// For each individual type, path and filename within the output directory can be changed using
/// `#[py(export_to = "...")]`. By default, the filename will be derived from the name of the type.
///
/// If, for some reason, you need to do this during runtime or cannot use `#[py(export)]`, bindings
/// can be exported manually:
///
/// | Function              | Includes Dependencies | To                 |
/// |-----------------------|-----------------------|--------------------|
/// | [`PY::export`]        | ❌                    | `PY_RS_EXPORT_DIR` |
/// | [`PY::export_all`]    | ✔️                    | `PY_RS_EXPORT_DIR` |
/// | [`PY::export_all_to`] | ✔️                    | _custom_           |
///
/// ### serde compatibility
/// By default, the feature `serde-compat` is enabled.
/// py-rs then parses serde attributes and adjusts the generated python bindings accordingly.
/// Not all serde attributes are supported yet - if you use an unsupported attribute, you'll see a
/// warning.
///
/// ### container attributes
/// attributes applicable for both structs and enums
///
/// - **`#[py(crate = "..")]`**
///   Generates code which references the module passed to it instead of defaulting to `::py_rs`
///   This is useful for cases where you have to re-export the crate.
///
/// - **`#[py(export)]`**  
///   Generates a test which will export the type, by default to `bindings/<name>.py` when running
///   `cargo test`. The default base directory can be overridden with the `PY_RS_EXPORT_DIR` environment variable.
///   Adding the variable to a project's [config.toml](https://doc.rust-lang.org/cargo/reference/config.html#env) can
///   make it easier to manage.
///   ```toml
///   # <project-root>/.cargo/config.toml
///   [env]
///   PY_RS_EXPORT_DIR = { value = "<OVERRIDE_DIR>", relative = true }
///   ```
///   <br/>
///
/// - **`#[py(export_to = "..")]`**  
///   Specifies where the type should be exported to. Defaults to `<name>.py`.  
///   The path given to the `export_to` attribute is relative to the `PY_RS_EXPORT_DIR` environment variable,
///   or, if `PY_RS_EXPORT_DIR` is not set, to `./bindings`  
///   If the provided path ends in a trailing `/`, it is interpreted as a directory.   
///   Note that you need to add the `export` attribute as well, in order to generate a test which exports the type.
///   <br/><br/>
///
/// - **`#[py(as = "..")]`**  
///   Overrides the type used in Python, using the provided Rust type instead.
///   This is useful when you have a custom serializer and deserializer and don't want to implement `PY` manually
///   <br/><br/>
///
/// - **`#[py(type = "..")]`**  
///   Overrides the type used in Python.  
///   This is useful when you have a custom serializer and deserializer and don't want to implement `PY` manually
///   <br/><br/>
///
/// - **`#[py(rename = "..")]`**  
///   Sets the python name of the generated type
///   <br/><br/>
///
/// - **`#[py(rename_all = "..")]`**  
///   Rename all fields/variants of the type.  
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case" and "SCREAMING-KEBAB-CASE"
///   <br/><br/>
///
/// - **`#[py(concrete(..)]`**  
///   Disables one ore more generic type parameters by specifying a concrete type for them.  
///   The resulting Python definition will not be generic over these parameters and will use the
///   provided type instead.  
///   This is especially useful for generic types containing associated types. Since Python does
///   not have an equivalent construct to associated types, we cannot generate a generic definition
///   for them. Using `#[py(concrete(..)]`, we can however generate a non-generic definition.
///   Example:
///   ```
///   # use py_rs::PY;
///   ##[derive(PY)]
///   ##[py(concrete(I = std::vec::IntoIter<String>))]
///   struct SearchResult<I: Iterator>(Vec<I::Item>);
///   // will always generate `type SearchResult = list[str]`.
///   ```
///   <br/><br/>
///
/// - **`#[py(bound)]`**
///   Override the bounds generated on the `PY` implementation for this type. This is useful in
///   combination with `#[py(concrete)]`, when the type's generic parameters aren't directly used
///   in a field or variant.
///
///   Example:
///   ```
///   # use py_rs::PY;
///
///   trait Container {
///       type Value: PY;
///   }
///
///   struct MyContainer;
///
///   ##[derive(PY)]
///   struct MyValue;
///
///   impl Container for MyContainer {
///       type Value = MyValue;
///   }
///
///   ##[derive(PY)]
///   ##[py(export, concrete(C = MyContainer))]
///   struct Inner<C: Container> {
///       value: C::Value,
///   }
///
///   ##[derive(PY)]
///   // Without `#[py(bound)]`, `#[derive(PY)]` would generate an unnecessary
///   // `C: PY` bound
///   ##[py(export, concrete(C = MyContainer), bound = "C::Value: PY")]
///   struct Outer<C: Container> {
///       inner: Inner<C>,
///   }
///   ```
///   <br/><br/>
///
/// ### struct attributes
/// - **`#[py(tag = "..")]`**  
///   Include the structs name (or value of `#[py(rename = "..")]`) as a field with the given key.
///   <br/><br/>
///
/// ### struct field attributes
///
/// - **`#[py(type = "..")]`**  
///   Overrides the type used in Python.  
///   This is useful when there's a type for which you cannot derive `PY`.
///   <br/><br/>
///
/// - **`#[py(as = "..")]`**  
///   Overrides the type of the annotated field, using the provided Rust type instead.
///   This is useful when there's a type for which you cannot derive `PY`.  
///   `_` may be used to refer to the type of the field, e.g `#[py(as = "Option<_>")]`.
///   <br/><br/>
///
/// - **`#[py(rename = "..")]`**  
///   Renames this field. To rename all fields of a struct, see the container attribute `#[py(rename_all = "..")]`.
///   <br/><br/>
///
/// - **`#[py(inline)]`**  
///   Inlines the type of this field, replacing its name with its definition.
///   <br/><br/>
///
/// - **`#[py(skip)]`**  
///   Skips this field, omitting it from the generated *Python* type.
///   <br/><br/>
///
/// - **`#[py(optional)]`**  
///   May be applied on a struct field of type `Option<T>`. By default, such a field would turn into `t: T | None`.  
///   If `#[py(optional)]` is present, `t?: T` is generated instead.  
///   If `#[py(optional = nullable)]` is present, `t?: T | None` is generated.
///   <br/><br/>
///
/// - **`#[py(flatten)]`**  
///   Flatten this field, inlining all the keys of the field's type into its parent.
///   <br/><br/>
///   
/// ### enum attributes
///
/// - **`#[py(tag = "..")]`**  
///   Changes the representation of the enum to store its tag in a separate field.  
///   See [the serde docs](https://serde.rs/enum-representations.html) for more information.
///   <br/><br/>
///
/// - **`#[py(content = "..")]`**  
///   Changes the representation of the enum to store its content in a separate field.  
///   See [the serde docs](https://serde.rs/enum-representations.html) for more information.
///   <br/><br/>
///
/// - **`#[py(untagged)]`**  
///   Changes the representation of the enum to not include its tag.  
///   See [the serde docs](https://serde.rs/enum-representations.html) for more information.
///   <br/><br/>
///
/// - **`#[py(rename_all = "..")]`**  
///   Rename all variants of this enum.  
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case" and "SCREAMING-KEBAB-CASE"
///   <br/><br/>
///
/// - **`#[py(rename_all_fields = "..")]`**  
///   Renames the fields of all the struct variants of this enum. This is equivalent to using
///   `#[py(rename_all = "..")]` on all of the enum's variants.
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case" and "SCREAMING-KEBAB-CASE"
///   <br/><br/>
///  
/// ### enum variant attributes
///
/// - **`#[py(rename = "..")]`**  
///   Renames this variant. To rename all variants of an enum, see the container attribute `#[py(rename_all = "..")]`.
///   <br/><br/>
///
/// - **`#[py(skip)]`**  
///   Skip this variant, omitting it from the generated *Python* type.
///   <br/><br/>
///
/// - **`#[py(untagged)]`**  
///   Changes this variant to be treated as if the enum was untagged, regardless of the enum's tag
///   and content attributes
///   <br/><br/>
///
/// - **`#[py(rename_all = "..")]`**  
///   Renames all the fields of a struct variant.
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case" and "SCREAMING-KEBAB-CASE"
///   <br/><br/>
pub trait PY {
    /// If this type does not have generic parameters, then `WithoutGenerics` should just be `Self`.
    /// If the type does have generic parameters, then all generic parameters must be replaced with
    /// a dummy type, e.g `py_rs::Dummy` or `()`.
    /// The only requirement for these dummy types is that `EXPORT_TO` must be `None`.
    ///
    /// # Example:
    /// ```
    /// use py_rs::PY;
    /// struct GenericType<A, B>(A, B);
    /// impl<A, B> PY for GenericType<A, B> {
    ///     type WithoutGenerics = GenericType<py_rs::Dummy, py_rs::Dummy>;
    ///     // ...
    ///     # fn decl() -> String { todo!() }
    ///     # fn decl_concrete() -> String { todo!() }
    ///     # fn name() -> String { todo!() }
    ///     # fn inline() -> String { todo!() }
    ///     # fn inline_flattened() -> String { todo!() }
    /// }
    /// ```
    type WithoutGenerics: PY + ?Sized;

    /// comment to describe this type in Python - when `PY` is derived, docs are
    /// automatically read from your doc comments or `#[doc = ".."]` attributes
    const DOCS: Option<&'static str> = None;

    /// Identifier of this type, excluding generic parameters.
    fn ident() -> String {
        // by default, fall back to `PY::name()`.
        let name = Self::name();

        match name.find('<') {
            // TODO check if this needs to be changed. I guess this is because of things like Array<Number>
            Some(i) => name[..i].to_owned(),
            None => name,
        }
    }

    /// Generate the classes for variants of enums
    /// Python does not support nested enum types like Rust
    /// nor are Union types well supported like in TypeScript
    /// For convinience we generate a enum with all the types
    /// of the enum as a class without the nesting.
    /// # Example:
    /// ```
    /// #[derive(Serialize, PY)]
    /// #[serde(tag = "kind", content = "data")]
    /// #[py(export)]
    /// enum ComplexEnum {
    ///     A,
    ///     B { foo: String, bar: f64 },
    ///     W(SimpleEnum),
    ///     F { nested: SimpleEnum },
    ///     V(Vec<Series>),
    ///     U(Box<User>),
    /// }
    /// ```
    /// ```python
    /// class ComplexEnumIdentifier(Enum):
    ///     A = "A"
    ///     B = "B"
    ///     W = "W"
    ///     F = "F"
    ///     V = "V"
    ///     U = "U"
    ///
    /// ...
    ///
    ///  ```
    fn variant_classes_decl() -> String;

    /// Declaration of this type, e.g. `type User = { user_id: number, ... }`. TODO update
    /// This function will panic if the type has no declaration.
    ///
    /// If this type is generic, then all provided generic parameters will be swapped for
    /// placeholders, resulting in a generic python definition.
    /// Both `SomeType::<i32>::decl()` and `SomeType::<String>::decl()` will therefore result in
    /// the same Python declaration `T = TypeVar('T')\n type SomeType[T] = ...`.
    fn decl() -> String;

    /// Declaration of this type using the supplied generic arguments.
    /// The resulting Python definition will not be generic. For that, see `PY::decl()`.
    /// If this type is not generic, then this function is equivalent to `PY::decl()`.
    fn decl_concrete() -> String;

    /// Name of this type in Python, including generic parameters
    fn name() -> String;

    /// Formats this types definition in Python, e.g `{ user_id: number }`.
    /// This function will panic if the type cannot be inlined.
    fn inline() -> String;

    /// Flatten a type declaration.  
    /// This function will panic if the type cannot be flattened.
    fn inline_flattened() -> String;

    /// Iterates over all dependency of this type.
    fn visit_dependencies(_: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
    }

    /// Iterates over all type parameters of this type.
    fn visit_generics(_: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
    }

    /// Resolves all dependencies of this type recursively.
    fn dependencies() -> Vec<Dependency>
    where
        Self: 'static,
    {
        let mut deps: Vec<Dependency> = vec![];
        struct Visit<'a>(&'a mut Vec<Dependency>);
        impl<'a> TypeVisitor for Visit<'a> {
            fn visit<T: PY + 'static + ?Sized>(&mut self) {
                if let Some(dep) = Dependency::from_ty::<T>() {
                    self.0.push(dep);
                }
            }
        }
        Self::visit_dependencies(&mut Visit(&mut deps));

        deps
    }

    /// Manually export this type to the filesystem.
    /// To export this type together with all of its dependencies, use [`PY::export_all`].
    ///
    /// # Automatic Exporting
    /// Types annotated with `#[py(export)]`, together with all of their dependencies, will be
    /// exported automatically whenever `cargo test` is run.  
    /// In that case, there is no need to manually call this function.
    ///
    /// # Target Directory
    /// The target directory to which the type will be exported may be changed by setting the
    /// `PY_RS_EXPORT_DIR` environment variable. By default, `./bindings` will be used.
    ///
    /// To specify a target directory manually, use [`PY::export_all_to`], which also exports all
    /// dependencies.
    ///
    /// To alter the filename or path of the type within the target directory,
    /// use `#[py(export_to = "...")]`.
    fn export() -> Result<(), ExportError>
    where
        Self: 'static,
    {
        let path = Self::default_output_path()
            .ok_or_else(std::any::type_name::<Self>)
            .map_err(ExportError::CannotBeExported)?;

        export::export_to::<Self, _>(path)
    }

    /// Manually export this type to the filesystem, together with all of its dependencies.  
    /// To export only this type, without its dependencies, use [`PY::export`].
    ///
    /// # Automatic Exporting
    /// Types annotated with `#[py(export)]`, together with all of their dependencies, will be
    /// exported automatically whenever `cargo test` is run.  
    /// In that case, there is no need to manually call this function.
    ///
    /// # Target Directory
    /// The target directory to which the types will be exported may be changed by setting the
    /// `PY_RS_EXPORT_DIR` environment variable. By default, `./bindings` will be used.
    ///
    /// To specify a target directory manually, use [`PY::export_all_to`].
    ///
    /// To alter the filenames or paths of the types within the target directory,
    /// use `#[py(export_to = "...")]`.
    fn export_all() -> Result<(), ExportError>
    where
        Self: 'static,
    {
        export::export_all_into::<Self>(&*export::default_out_dir())
    }

    /// Manually export this type into the given directory, together with all of its dependencies.  
    /// To export only this type, without its dependencies, use [`PY::export`].
    ///
    /// Unlike [`PY::export_all`], this function disregards `PY_RS_EXPORT_DIR`, using the provided
    /// directory instead.
    ///
    /// To alter the filenames or paths of the types within the target directory,
    /// use `#[py(export_to = "...")]`.
    ///
    /// # Automatic Exporting
    /// Types annotated with `#[py(export)]`, together with all of their dependencies, will be
    /// exported automatically whenever `cargo test` is run.  
    /// In that case, there is no need to manually call this function.
    fn export_all_to(out_dir: impl AsRef<Path>) -> Result<(), ExportError>
    where
        Self: 'static,
    {
        export::export_all_into::<Self>(out_dir)
    }

    /// Manually generate bindings for this type, returning a [`String`].  
    /// This function does not format the output, even if the `format` feature is enabled.
    ///
    /// # Automatic Exporting
    /// Types annotated with `#[py(export)]`, together with all of their dependencies, will be
    /// exported automatically whenever `cargo test` is run.  
    /// In that case, there is no need to manually call this function.
    fn export_to_string() -> Result<String, ExportError>
    where
        Self: 'static,
    {
        export::export_to_string::<Self>()
    }

    /// Returns the output path to where `T` should be exported.  
    /// The returned path does _not_ include the base directory from `PY_RS_EXPORT_DIR`.  
    ///
    /// To get the output path containing `PY_RS_EXPORT_DIR`, use [`PY::default_output_path`].
    ///
    /// When deriving `PY`, the output path can be altered using `#[py(export_to = "...")]`.  
    /// See the documentation of [`PY`] for more details.
    ///
    /// The output of this function depends on the environment variable `PY_RS_EXPORT_DIR`, which is
    /// used as base directory. If it is not set, `./bindings` is used as default directory.
    ///
    /// If `T` cannot be exported (e.g because it's a primitive type), this function will return
    /// `None`.
    fn output_path() -> Option<&'static Path> {
        None
    }

    /// Returns the output path to where `T` should be exported.  
    ///
    /// The output of this function depends on the environment variable `PY_RS_EXPORT_DIR`, which is
    /// used as base directory. If it is not set, `./bindings` is used as default directory.
    ///
    /// To get the output path relative to `PY_RS_EXPORT_DIR` and without reading the environment
    /// variable, use [`PY::output_path`].
    ///
    /// When deriving `PY`, the output path can be altered using `#[py(export_to = "...")]`.  
    /// See the documentation of [`PY`] for more details.
    ///
    /// If `T` cannot be exported (e.g because it's a primitive type), this function will return
    /// `None`.
    fn default_output_path() -> Option<PathBuf> {
        Some(export::default_out_dir().join(Self::output_path()?))
    }
}

/// A visitor used to iterate over all dependencies or generics of a type.
/// When an instance of [`TypeVisitor`] is passed to [`PY::visit_dependencies`] or
/// [`PY::visit_generics`], the [`TypeVisitor::visit`] method will be invoked for every dependency
/// or generic parameter respectively.
pub trait TypeVisitor: Sized {
    fn visit<T: PY + 'static + ?Sized>(&mut self);
}

/// A python type which is depended upon by other types.
/// This information is required for generating the correct import statements.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dependency {
    /// Type ID of the rust type
    pub type_id: TypeId,
    /// Name of the type in Python
    pub py_name: String,
    /// Path to where the type would be exported. By default a filename is derived from the types
    /// name, which can be customized with `#[py(export_to = "..")]`.  
    /// This path does _not_ include a base directory.
    pub output_path: &'static Path,
}

impl Dependency {
    /// Constructs a [`Dependency`] from the given type `T`.
    /// If `T` is not exportable (meaning `T::EXPORT_TO` is `None`), this function will return
    /// `None`
    pub fn from_ty<T: PY + 'static + ?Sized>() -> Option<Self> {
        let output_path = T::output_path()?;
        Some(Dependency {
            type_id: TypeId::of::<T>(),
            py_name: T::ident(),
            output_path,
        })
    }
}

// generate impls for primitive types
macro_rules! impl_primitives {
    ($($($ty:ty),* => $l:literal),*) => { $($(
        impl PY for $ty {
            type WithoutGenerics = Self;
            fn name() -> String { $l.to_owned() }
            fn inline() -> String { <Self as $crate::PY>::name() }
            fn inline_flattened() -> String { panic!("{} cannot be flattened", <Self as $crate::PY>::name()) }
            fn decl() -> String { panic!("{} cannot be declared", <Self as $crate::PY>::name()) }
            fn decl_concrete() -> String { panic!("{} cannot be declared", <Self as $crate::PY>::name()) }
            fn variant_classes_decl() -> String { panic!("{} cannot be declared", <Self as $crate::PY>::name()) }
        }
    )*)* };
}
// generate impls for tuples
macro_rules! impl_tuples {
    ( impl $($i:ident),* ) => {
        impl<$($i: PY),*> PY for ($($i,)*) {
            type WithoutGenerics = (Dummy, );
            fn name() -> String {
                format!("tuple[{}]", [$(<$i as $crate::PY>::name()),*].join(", "))
            }
            fn inline() -> String {
                panic!("tuple cannot be inlined!");
            }
            fn visit_generics(v: &mut impl TypeVisitor)
            where
                Self: 'static
            {
                $(
                    v.visit::<$i>();
                    <$i>::visit_generics(v);
                )*
            }
            fn inline_flattened() -> String { panic!("tuple cannot be flattened") }
            fn decl() -> String { panic!("tuple cannot be declared") }
            fn decl_concrete() -> String { panic!("tuple cannot be declared") }
            fn variant_classes_decl() -> String { panic!("{} cannot be declared", <Self as $crate::PY>::name()) }
        }
    };
    ( $i2:ident $(, $i:ident)* ) => {
        impl_tuples!(impl $i2 $(, $i)* );
        impl_tuples!($($i),*);
    };
    () => {};
}

// generate impls for wrapper types
macro_rules! impl_wrapper {
    ($($t:tt)*) => {
        $($t)* {
            type WithoutGenerics = Self;
            fn name() -> String { T::name() }
            fn inline() -> String { T::inline() }
            fn inline_flattened() -> String { T::inline_flattened() }
            fn visit_dependencies(v: &mut impl TypeVisitor)
            where
                Self: 'static,
            {
                T::visit_dependencies(v);
            }

            fn visit_generics(v: &mut impl TypeVisitor)
            where
                Self: 'static,
            {
                T::visit_generics(v);
                v.visit::<T>();
            }
            fn decl() -> String { panic!("wrapper type cannot be declared") }
            fn decl_concrete() -> String { panic!("wrapper type cannot be declared") }
            fn variant_classes_decl() -> String { panic!("{} cannot be declared", <Self as $crate::PY>::name()) }
        }
    };
}

// implement PY for the $shadow, deferring to the impl $s
macro_rules! impl_shadow {
    (as $s:ty: $($impl:tt)*) => {
        $($impl)* {
            type WithoutGenerics = <$s as $crate::PY>::WithoutGenerics;
            fn ident() -> String { <$s as $crate::PY>::ident() }
            fn name() -> String { <$s as $crate::PY>::name() }
            fn inline() -> String { <$s as $crate::PY>::inline() }
            fn inline_flattened() -> String { <$s as $crate::PY>::inline_flattened() }
            fn visit_dependencies(v: &mut impl $crate::TypeVisitor)
            where
                Self: 'static,
            {
                <$s as $crate::PY>::visit_dependencies(v);
            }
            fn visit_generics(v: &mut impl $crate::TypeVisitor)
            where
                Self: 'static,
            {
                <$s as $crate::PY>::visit_generics(v);
            }
            fn decl() -> String { <$s as $crate::PY>::decl() }
            fn decl_concrete() -> String { <$s as $crate::PY>::decl_concrete() }
            fn output_path() -> Option<&'static std::path::Path> { <$s as $crate::PY>::output_path() }
            fn variant_classes_decl() -> String { panic!("{} cannot be declared", <Self as $crate::PY>::name()) }
        }
    };
}

impl<T: PY> PY for Option<T> {
    type WithoutGenerics = Self;

    fn name() -> String {
        format!("{} | None", T::name())
    }

    fn inline() -> String {
        format!("{} | None", T::inline())
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        T::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        T::visit_generics(v);
        v.visit::<T>();
    }

    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }
    fn variant_classes_decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }
}

impl<T: PY, E: PY> PY for Result<T, E> {
    // TODO update
    // maybe just not support this or add the result import from python
    type WithoutGenerics = Result<Dummy, Dummy>;

    fn name() -> String {
        format!("{{ Ok : {} }} | {{ Err : {} }}", T::name(), E::name())
    }

    fn inline() -> String {
        format!("{{ Ok : {} }} | {{ Err : {} }}", T::inline(), E::inline())
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        T::visit_dependencies(v);
        E::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        T::visit_generics(v);
        v.visit::<T>();
        E::visit_generics(v);
        v.visit::<E>();
    }

    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }
    fn variant_classes_decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }
}

impl<T: PY> PY for Vec<T> {
    type WithoutGenerics = Vec<Dummy>;

    fn ident() -> String {
        "list".to_owned()
    }

    fn name() -> String {
        format!("list[{}]", T::name())
    }

    fn inline() -> String {
        format!("list[{}]", T::inline())
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        T::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        T::visit_generics(v);
        v.visit::<T>();
    }

    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }
    fn variant_classes_decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }
}

// Arrays longer than this limit will be emitted as Array<T>
const ARRAY_TUPLE_LIMIT: usize = 64;
impl<T: PY, const N: usize> PY for [T; N] {
    type WithoutGenerics = [Dummy; N];
    fn name() -> String {
        if N > ARRAY_TUPLE_LIMIT {
            return Vec::<T>::name();
        }

        format!(
            "[{}]",
            (0..N).map(|_| T::name()).collect::<Box<[_]>>().join(", ")
        )
    }

    fn inline() -> String {
        if N > ARRAY_TUPLE_LIMIT {
            return Vec::<T>::inline();
        }

        format!(
            "[{}]",
            (0..N).map(|_| T::inline()).collect::<Box<[_]>>().join(", ")
        )
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        T::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        T::visit_generics(v);
        v.visit::<T>();
    }

    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }
    fn variant_classes_decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }
}

impl<K: PY, V: PY, H> PY for HashMap<K, V, H> {
    type WithoutGenerics = HashMap<Dummy, Dummy>;

    fn ident() -> String {
        panic!()
    }

    fn name() -> String {
        format!("{{ [key in {}]?: {} }}", K::name(), V::name()) // TODO update
    }

    fn inline() -> String {
        format!("dict[{}, {}]", K::inline(), V::inline())
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        K::visit_dependencies(v);
        V::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        K::visit_generics(v);
        v.visit::<K>();
        V::visit_generics(v);
        v.visit::<V>();
    }

    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }
    fn variant_classes_decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }
}

// TODO should not be supported
impl<I: PY> PY for Range<I> {
    type WithoutGenerics = Range<Dummy>;
    fn name() -> String {
        format!("{{ start: {}, end: {}, }}", I::name(), I::name())
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        I::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        I::visit_generics(v);
        v.visit::<I>();
    }

    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn inline() -> String {
        panic!("{} cannot be inlined", Self::name())
    }

    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }
    fn variant_classes_decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }
}

impl_shadow!(as Range<I>: impl<I: PY> PY for RangeInclusive<I>);
impl_shadow!(as Vec<T>: impl<T: PY, H> PY for HashSet<T, H>);
impl_shadow!(as Vec<T>: impl<T: PY> PY for BTreeSet<T>);
impl_shadow!(as HashMap<K, V>: impl<K: PY, V: PY> PY for BTreeMap<K, V>);
impl_shadow!(as Vec<T>: impl<T: PY> PY for [T]);

impl_wrapper!(impl<T: PY + ?Sized> PY for &T);
impl_wrapper!(impl<T: PY + ?Sized> PY for Box<T>);
impl_wrapper!(impl<T: PY + ?Sized> PY for std::sync::Arc<T>);
impl_wrapper!(impl<T: PY + ?Sized> PY for std::rc::Rc<T>);
impl_wrapper!(impl<'a, T: PY + ToOwned + ?Sized> PY for std::borrow::Cow<'a, T>);
impl_wrapper!(impl<T: PY> PY for std::cell::Cell<T>);
impl_wrapper!(impl<T: PY> PY for std::cell::RefCell<T>);
impl_wrapper!(impl<T: PY> PY for std::sync::Mutex<T>);
impl_wrapper!(impl<T: PY> PY for std::sync::RwLock<T>);
impl_wrapper!(impl<T: PY + ?Sized> PY for std::sync::Weak<T>);
impl_wrapper!(impl<T: PY> PY for std::marker::PhantomData<T>);

impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

#[cfg(feature = "bigdecimal-impl")]
impl_primitives! { bigdecimal::BigDecimal => "str" }

#[cfg(feature = "smol_str-impl")]
impl_primitives! { smol_str::SmolStr => "str" }

#[cfg(feature = "uuid-impl")]
impl_primitives! { uuid::Uuid => "str" }

#[cfg(feature = "url-impl")]
impl_primitives! { url::Url => "str" }

#[cfg(feature = "ordered-float-impl")]
impl_primitives! { ordered_float::OrderedFloat<f32> => "float" }

#[cfg(feature = "ordered-float-impl")]
impl_primitives! { ordered_float::OrderedFloat<f64> => "float" }

#[cfg(feature = "bson-uuid-impl")]
impl_primitives! { bson::oid::ObjectId => "str" }

#[cfg(feature = "bson-uuid-impl")]
impl_primitives! { bson::Uuid => "str" }

#[cfg(feature = "indexmap-impl")]
impl_shadow!(as Vec<T>: impl<T: PY> PY for indexmap::IndexSet<T>);

#[cfg(feature = "indexmap-impl")]
impl_shadow!(as HashMap<K, V>: impl<K: PY, V: PY> PY for indexmap::IndexMap<K, V>);

#[cfg(feature = "heapless-impl")]
impl_shadow!(as Vec<T>: impl<T: PY, const N: usize> PY for heapless::Vec<T, N>);

#[cfg(feature = "semver-impl")]
impl_primitives! { semver::Version => "str" }

#[cfg(feature = "bytes-impl")]
mod bytes {
    use super::PY;

    impl_shadow!(as Vec<u8>: impl PY for bytes::Bytes);
    impl_shadow!(as Vec<u8>: impl PY for bytes::BytesMut);
}

impl_primitives! {
    u8, i8, NonZeroU8, NonZeroI8,
    u16, i16, NonZeroU16, NonZeroI16,
    u32, i32, NonZeroU32, NonZeroI32,
    usize, isize, NonZeroUsize, NonZeroIsize,
    u64, i64, NonZeroU64, NonZeroI64,
    u128, i128, NonZeroU128, NonZeroI128 => "int",
    f32, f64 => "float",
    bool => "bool",
    char, Path, PathBuf, String, str,
    Ipv4Addr, Ipv6Addr, IpAddr, SocketAddrV4, SocketAddrV6, SocketAddr => "str",
    () => "None"
}

#[rustfmt::skip]
pub(crate) use impl_primitives;
#[rustfmt::skip]
pub(crate) use impl_shadow;
#[rustfmt::skip]
pub(crate) use impl_wrapper;

#[doc(hidden)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dummy;

impl std::fmt::Display for Dummy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PY for Dummy {
    type WithoutGenerics = Self;
    fn name() -> String {
        "Dummy".to_owned()
    }

    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn inline() -> String {
        panic!("{} cannot be inlined", Self::name())
    }

    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }

    fn variant_classes_decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }
}
