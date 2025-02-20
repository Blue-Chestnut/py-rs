#![allow(dead_code)]

use serde::Serialize;
use py_rs::PY;

#[derive(Serialize, PY)]
#[py(export, export_to = "optional_field/")]
struct OptionalInStruct {
    #[py(optional)]
    a: Option<i32>,
    #[py(optional = nullable)]
    b: Option<i32>,
    c: Option<i32>,
}

#[test]
fn in_struct() {
    let a = "a?: number";
    let b = "b?: number | null";
    let c = "c: number | null";
    assert_eq!(OptionalInStruct::inline(), format!("{{ {a}, {b}, {c}, }}"));
}

#[derive(Serialize, PY)]
#[py(export, export_to = "optional_field/")]
enum OptionalInEnum {
    A {
        #[py(optional)]
        a: Option<i32>,
    },
    B {
        b: Option<String>,
    },
}

#[test]
fn in_enum() {
    assert_eq!(
        OptionalInEnum::inline(),
        r#"{ "A": { a?: number, } } | { "B": { b: string | null, } }"#
    );
}

#[derive(Serialize, PY)]
#[py(export, export_to = "optional_field/")]
struct OptionalFlatten {
    #[py(optional)]
    a: Option<i32>,
    #[py(optional = nullable)]
    b: Option<i32>,
    c: Option<i32>,
}

#[derive(Serialize, PY)]
#[py(export, export_to = "optional_field/")]
struct Flatten {
    #[py(flatten)]
    x: OptionalFlatten,
}

#[test]
fn flatten() {
    assert_eq!(Flatten::inline(), OptionalFlatten::inline());
}

#[derive(Serialize, PY)]
#[py(export, export_to = "optional_field/")]
struct OptionalInline {
    #[py(optional)]
    a: Option<i32>,
    #[py(optional = nullable)]
    b: Option<i32>,
    c: Option<i32>,
}

#[derive(Serialize, PY)]
#[py(export, export_to = "optional_field/")]
struct Inline {
    #[py(inline)]
    x: OptionalInline,
}

#[test]
fn inline() {
    let a = "a?: number";
    let b = "b?: number | null";
    let c = "c: number | null";
    assert_eq!(Inline::inline(), format!("{{ x: {{ {a}, {b}, {c}, }}, }}"));
}
