#![allow(dead_code)]

use std::time::Instant;

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use py_rs::PY;

struct Unsupported<T>(T);
struct Unsupported2;

#[derive(PY)]
#[py(export, export_to = "type_override/")]
struct Override {
    a: i32,
    #[py(type = "0 | 1 | 2")]
    b: i32,
    #[py(type = "string")]
    x: Instant,
    #[py(type = "string")]
    y: Unsupported<Unsupported<Unsupported2>>,
    #[py(type = "string | null")]
    z: Option<Unsupported2>,
}

#[test]
fn simple() {
    assert_eq!(
        Override::inline(),
        "{ a: number, b: 0 | 1 | 2, x: string, y: string, z: string | null, }"
    )
}

#[derive(PY)]
#[py(export, export_to = "type_override/")]
struct New1(#[py(type = "string")] Unsupported2);

#[derive(PY)]
#[py(export, export_to = "type_override/")]
struct New2(#[py(type = "string | null")] Unsupported<Unsupported2>);

#[test]
fn newtype() {
    assert_eq!(New1::inline(), r#"string"#);
    assert_eq!(New2::inline(), r#"string | null"#);
}

#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct S;

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "t"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "t"))]
#[py(export, export_to = "type_override/")]
enum Internal {
    Newtype(#[py(type = "unknown")] S),
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "t", content = "c"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "t", content = "c"))]
#[py(export, export_to = "type_override/")]
enum Adjacent {
    Newtype(#[py(type = "unknown")] S),
}

#[test]
fn enum_newtype_representations() {
    // regression test for https://github.com/Aleph-Alpha/ts-rs/issues/126
    assert_eq!(Internal::inline(), r#"{ "t": "Newtype" } & unknown"#);
    assert_eq!(Adjacent::inline(), r#"{ "t": "Newtype", "c": unknown }"#);
}
