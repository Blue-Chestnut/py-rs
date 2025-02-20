#![allow(dead_code, clippy::disallowed_names)]

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use py_rs::PY;

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "type"))]
#[py(export, export_to = "union_with_internal_tag/")]
enum EnumWithInternalTag {
    A { foo: String },
    B { bar: i32 },
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "union_with_internal_tag/")]
struct InnerA {
    foo: String,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "union_with_internal_tag/")]
struct InnerB {
    bar: i32,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "type"))]
#[py(export, export_to = "union_with_internal_tag/")]
enum EnumWithInternalTag2 {
    A(InnerA),
    B(InnerB),
}

#[test]
fn test_enums_with_internal_tags() {
    assert_eq!(
        EnumWithInternalTag::decl(),
        r#"type EnumWithInternalTag = { "type": "A", foo: string, } | { "type": "B", bar: number, };"#
    );

    assert_eq!(
        EnumWithInternalTag2::decl(),
        r#"type EnumWithInternalTag2 = { "type": "A" } & InnerA | { "type": "B" } & InnerB;"#
    );
}
