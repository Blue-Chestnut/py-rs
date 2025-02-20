#![allow(dead_code)]

use std::{
    cell::UnsafeCell, mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicPtr, time::Instant,
};

use py_rs::PY;
use serde::Serialize;

type Unsupported = UnsafeCell<MaybeUninit<NonNull<AtomicPtr<i32>>>>;

#[derive(PY)]
#[py(export, export_to = "type_as/")]
struct ExternalTypeDef {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(PY)]
#[py(export, export_to = "type_as/")]
struct Override {
    a: i32,
    #[py(as = "ExternalTypeDef")]
    #[py(inline)]
    x: Instant,
    // here, 'as' just behaves like 'type' (though it adds a dependency!)
    #[py(as = "ExternalTypeDef")]
    y: Unsupported,
    #[py(as = "(i32, ExternalTypeDef, i32)")]
    z: Unsupported,
}

#[test]
fn struct_properties() {
    assert_eq!(
        Override::inline(),
        "{ \
           a: number, \
           x: { a: number, b: number, c: number, }, \
           y: ExternalTypeDef, \
           z: [number, ExternalTypeDef, number], \
        }"
    );
    assert!(Override::dependencies()
        .iter()
        .any(|d| d.py_name == "ExternalTypeDef"));
}

#[derive(PY)]
#[py(export, export_to = "type_as/")]
enum OverrideEnum {
    A(#[py(as = "ExternalTypeDef")] Instant),
    B {
        #[py(as = "ExternalTypeDef")]
        x: Unsupported,
        y: i32,
        z: i32,
    },
}

mod deser {
    use serde::{Serialize, Serializer};

    use super::Instant;
    pub fn serialize<S: Serializer>(field: &Instant, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Foo {
            x: i32,
        }
        Foo { x: 0 }.serialize(serializer)
    }
}

#[derive(PY)]
struct OverrideVariantDef {
    x: i32,
}

#[derive(PY, Serialize)]
#[py(export, export_to = "type_as/")]
enum OverrideVariant {
    #[py(as = "OverrideVariantDef")]
    #[serde(with = "deser")]
    A {
        x: Instant,
    },
    B {
        y: i32,
        z: i32,
    },
}

#[test]
fn enum_varianpy() {
    let a = OverrideVariant::A { x: Instant::now() };
    assert_eq!(serde_json::to_string(&a).unwrap(), r#"{"A":{"x":0}}"#);
    assert_eq!(
        OverrideEnum::inline(),
        r#"{ "A": ExternalTypeDef } | { "B": { x: ExternalTypeDef, y: number, z: number, } }"#
    );

    assert_eq!(
        OverrideVariant::inline(),
        r#"{ "A": OverrideVariantDef } | { "B": { y: number, z: number, } }"#
    );
}

#[derive(PY)]
#[py(export, export_to = "type_as/")]
struct Outer {
    #[py(as = "Option<ExternalTypeDef>")]
    #[py(optional = nullable, inline)]
    x: Unsupported,
    #[py(as = "Option<ExternalTypeDef>")]
    #[py(optional = nullable)]
    y: Unsupported,
}

#[test]
fn complex() {
    let external = ExternalTypeDef::inline();
    assert_eq!(
        Outer::inline(),
        format!(r#"{{ x?: {external} | null, y?: ExternalTypeDef | null, }}"#)
    )
}
