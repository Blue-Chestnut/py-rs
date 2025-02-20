#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use py_rs::PY;

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening/externally_tagged/")]
struct FooExternally {
    qux: i32,
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    baz: BarExternally,
    biz: Option<String>,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening/externally_tagged/")]
enum BarExternally {
    Baz { a: i32, a2: String },
    Biz { b: bool },
    Buz { c: String, d: Option<i32> },
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "enum_flattening/externally_tagged/")]
struct NestedExternally {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    a: FooExternally,
    u: u32,
}

#[test]
fn externally_tagged() {
    assert_eq!(
        FooExternally::inline(),
        r#"{ qux: number, biz: string | null, } & ({ "Baz": { a: number, a2: string, } } | { "Biz": { b: boolean, } } | { "Buz": { c: string, d: number | null, } })"#
    );
    assert_eq!(
        NestedExternally::inline(),
        r#"{ u: number, qux: number, biz: string | null, } & ({ "Baz": { a: number, a2: string, } } | { "Biz": { b: boolean, } } | { "Buz": { c: string, d: number | null, } })"#
    );
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening/adjacently_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct FooAdjecently {
    one: i32,
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    baz: BarAdjecently,
    qux: Option<String>,
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening/adjacently_tagged/")]
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

#[test]
fn adjacently_tagged() {
    assert_eq!(
        FooAdjecently::inline(),
        r#"{ one: number, qux: string | null, } & ({ "type": "Baz", "stuff": { a: number, a2: string, } } | { "type": "Biz", "stuff": { b: boolean, } } | { c: string, d: number | null, })"#
    );
    assert_eq!(
        NestedAdjecently::inline(),
        r#"{ u: number, one: number, qux: string | null, } & ({ "type": "Baz", "stuff": { a: number, a2: string, } } | { "type": "Biz", "stuff": { b: boolean, } } | { c: string, d: number | null, })"#
    );
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening/internally_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct FooInternally {
    qux: Option<String>,

    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    baz: BarInternally,
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening/internally_tagged/")]
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

#[test]
fn internally_tagged() {
    assert_eq!(
        FooInternally::inline(),
        r#"{ qux: string | null, } & ({ "type": "Baz", a: number, a2: string, } | { "type": "Biz", b: boolean, } | { "type": "Buz", c: string, d: number | null, })"#
    );
    assert_eq!(
        NestedInternally::inline(),
        r#"{ u: number, qux: string | null, } & ({ "type": "Baz", a: number, a2: string, } | { "type": "Biz", b: boolean, } | { "type": "Buz", c: string, d: number | null, })"#
    );
}

#[derive(PY)]
#[py(export, export_to = "enum_flattening/untagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct FooUntagged {
    one: u32,
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), py(flatten))]
    baz: BarUntagged,
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
#[py(export, export_to = "enum_flattening/untagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(untagged))]
#[cfg_attr(not(feature = "serde-compat"), py(untagged))]
enum BarUntagged {
    Baz { a: i32, a2: String },
    Biz { b: bool },
    Buz { c: String },
}

#[test]
fn untagged() {
    assert_eq!(
        FooUntagged::inline(),
        r#"{ one: number, } & ({ a: number, a2: string, } | { b: boolean, } | { c: string, })"#
    );
    assert_eq!(
        NestedUntagged::inline(),
        r#"{ u: number, one: number, } & ({ a: number, a2: string, } | { b: boolean, } | { c: string, })"#
    );
}
