#![allow(dead_code)]

use py_rs::{Dependency, PY};
#[cfg(feature = "serde-compat")]
use serde::Serialize;

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "union_with_data/")]
struct Bar {
    field: i32,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "union_with_data/")]
struct Foo {
    bar: Bar,
}

#[derive(PY)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[py(export, export_to = "union_with_data/")]
enum SimpleEnum {
    A(String),
    B(i32),
    C,
    D(String, i32),
    E(Foo),
    F { a: i32, b: String },
}

#[test]
fn test_stateful_enum() {
    assert_eq!(Bar::decl(), r#"type Bar = { field: number, };"#);
    assert_eq!(Bar::dependencies(), vec![]);

    assert_eq!(Foo::decl(), r#"type Foo = { bar: Bar, };"#);
    assert_eq!(
        Foo::dependencies(),
        vec![Dependency::from_ty::<Bar>().unwrap()]
    );

    assert_eq!(
        SimpleEnum::decl(),
        r#"type SimpleEnum = { "A": string } | { "B": number } | "C" | { "D": [string, number] } | { "E": Foo } | { "F": { a: number, b: string, } };"#
    );
    assert!(SimpleEnum::dependencies()
        .into_iter()
        .all(|dep| dep == Dependency::from_ty::<Foo>().unwrap()),);
}
