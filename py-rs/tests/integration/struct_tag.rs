#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use py_rs::PY;

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "type"))]
struct TaggedType {
    a: i32,
    b: i32,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "type"))]
struct EmptyTaggedType {}

#[test]
fn test() {
    assert_eq!(
        TaggedType::inline(),
        "{ \"type\": \"TaggedType\", a: number, b: number, }"
    );

    assert_eq!(
        EmptyTaggedType::inline(),
        r#"{ "type": "EmptyTaggedType", }"#
    );
}
