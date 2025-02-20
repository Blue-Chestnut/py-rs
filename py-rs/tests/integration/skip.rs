#![allow(dead_code, unused_imports)]

use std::error::Error;

use serde::Serialize;
use py_rs::PY;

struct Unsupported;

#[derive(PY)]
#[py(export, export_to = "skip/")]
struct Skip {
    a: i32,
    b: i32,
    #[py(skip)]
    c: String,
    #[py(skip)]
    d: Box<dyn Error>,
}

#[test]
fn simple() {
    assert_eq!(Skip::inline(), "{ a: number, b: number, }");
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "skip/")]
enum Externally {
    A(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        Unsupported,
    ),
    B(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        Unsupported,
        i32,
    ),
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        x: Unsupported,
    },
    D {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        x: Unsupported,
        y: i32,
    },
}

#[test]
fn externally_tagged() {
    // TODO: variant C should probably not generate `{}`
    assert_eq!(
        Externally::decl(),
        r#"type Externally = "A" | { "B": [number] } | { "C": {  } } | { "D": { y: number, } };"#
    );
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "t"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "t"))]
#[py(export, export_to = "skip/")]
enum Internally {
    A(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        Unsupported,
    ),
    B {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        x: Unsupported,
    },
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        x: Unsupported,
        y: i32,
    },
}

#[test]
fn internally_tagged() {
    assert_eq!(
        Internally::decl(),
        r#"type Internally = { "t": "A" } | { "t": "B", } | { "t": "C", y: number, };"#
    );
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "t", content = "c"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "t", content = "c"))]
#[py(export, export_to = "skip/")]
enum Adjacently {
    A(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        Unsupported,
    ),
    B(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        Unsupported,
        i32,
    ),
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        x: Unsupported,
    },
    D {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), py(skip))]
        x: Unsupported,
        y: i32,
    },
}

#[test]
fn adjacently_tagged() {
    // TODO: variant C should probably not generate `{ .., "c": { } }`
    assert_eq!(
        Adjacently::decl(),
        r#"type Adjacently = { "t": "A" } | { "t": "B", "c": [number] } | { "t": "C", "c": {  } } | { "t": "D", "c": { y: number, } };"#
    );
}
