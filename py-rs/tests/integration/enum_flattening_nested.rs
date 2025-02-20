#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use py_rs::PY;

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct FooExternally {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    baz: BarExternally,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening_nested/externally_tagged/")]
enum BarExternally {
    Baz { a: i32, a2: String },
    Biz { b: bool },
    Buz { c: String, d: Option<i32> },
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedExternally {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    a: FooExternally,
    u: u32,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedExternallyLonely {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    a: FooExternally,
}

#[test]
fn externally_tagged() {
    // Notice here that baz is the only field inside `FooExternally`, so the parenthesis
    // aren't needed
    assert_eq!(
        FooExternally::inline(),
        r#"{ "Baz": { a: number, a2: string, } } | { "Biz": { b: boolean, } } | { "Buz": { c: string, d: number | null, } }"#
    );

    // But when flattening, the parenthesis are needed due to type intesections
    assert_eq!(
        NestedExternally::inline(),
        r#"{ u: number, } & ({ "Baz": { a: number, a2: string, } } | { "Biz": { b: boolean, } } | { "Buz": { c: string, d: number | null, } })"#
    );

    // And here, they are, again, unecessary
    assert_eq!(
        NestedExternallyLonely::inline(),
        r#"{ "Baz": { a: number, a2: string, } } | { "Biz": { b: boolean, } } | { "Buz": { c: string, d: number | null, } }"#
    );
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening_nested/adjacently_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct FooAdjecently {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    baz: BarAdjecently,
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening_nested/adjacently_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "stuff"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "type", content = "stuff"))]
enum BarAdjecently {
    Baz {
        a: i32,
        a2: String,
    },
    Biz {
        b: bool,
    },

    #[cfg_attr(feature = "serde-compat", serde(untagged))]
    #[cfg_attr(not(feature = "serde-compat"), py(untagged))]
    Buz {
        c: String,
        d: Option<i32>,
    },
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct NestedAdjecently {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    a: FooAdjecently,
    u: u32,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedAdjecentlyLonely {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    a: FooAdjecently,
}

#[test]
fn adjacently_tagged() {
    assert_eq!(
        FooAdjecently::inline(),
        r#"{ "type": "Baz", "stuff": { a: number, a2: string, } } | { "type": "Biz", "stuff": { b: boolean, } } | { c: string, d: number | null, }"#
    );

    assert_eq!(
        NestedAdjecently::inline(),
        r#"{ u: number, } & ({ "type": "Baz", "stuff": { a: number, a2: string, } } | { "type": "Biz", "stuff": { b: boolean, } } | { c: string, d: number | null, })"#
    );

    assert_eq!(
        NestedAdjecentlyLonely::inline(),
        r#"{ "type": "Baz", "stuff": { a: number, a2: string, } } | { "type": "Biz", "stuff": { b: boolean, } } | { c: string, d: number | null, }"#
    );
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening_nested/internally_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct FooInternally {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    baz: BarInternally,
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening_nested/internally_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), py(tag = "type"))]
enum BarInternally {
    Baz { a: i32, a2: String },
    Biz { b: bool },
    Buz { c: String, d: Option<i32> },
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct NestedInternally {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    a: FooInternally,
    u: u32,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedInternallyLonely {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    a: FooInternally,
}

#[test]
fn internally_tagged() {
    assert_eq!(
        FooInternally::inline(),
        r#"{ "type": "Baz", a: number, a2: string, } | { "type": "Biz", b: boolean, } | { "type": "Buz", c: string, d: number | null, }"#
    );

    assert_eq!(
        NestedInternally::inline(),
        r#"{ u: number, } & ({ "type": "Baz", a: number, a2: string, } | { "type": "Biz", b: boolean, } | { "type": "Buz", c: string, d: number | null, })"#
    );

    assert_eq!(
        NestedInternallyLonely::inline(),
        r#"{ "type": "Baz", a: number, a2: string, } | { "type": "Biz", b: boolean, } | { "type": "Buz", c: string, d: number | null, }"#
    );
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening_nested/untagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct FooUntagged {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    baz: BarUntagged,
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening_nested/untagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(untagged))]
#[cfg_attr(not(feature = "serde-compat"), py(untagged))]
enum BarUntagged {
    Baz { a: i32, a2: String },
    Biz { b: bool },
    Buz { c: String },
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct NestedUntagged {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    a: FooUntagged,
    u: u32,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedUntaggedLonely {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    a: FooUntagged,
}

#[test]
fn untagged() {
    assert_eq!(
        FooUntagged::inline(),
        r#"{ a: number, a2: string, } | { b: boolean, } | { c: string, }"#
    );

    assert_eq!(
        NestedUntagged::inline(),
        r#"{ u: number, } & ({ a: number, a2: string, } | { b: boolean, } | { c: string, })"#
    );

    assert_eq!(
        NestedUntaggedLonely::inline(),
        r#"{ a: number, a2: string, } | { b: boolean, } | { c: string, }"#
    );
}
