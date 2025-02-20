#![allow(dead_code)]

use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "flatten/")]
struct A {
    a: i32,
    b: i32,
}

#[derive(PY)]
#[py(export, export_to = "flatten/")]
struct B {
    #[py(flatten)]
    a: A,
    c: i32,
}

#[derive(PY)]
#[py(export, export_to = "flatten/")]
struct C {
    #[py(inline)]
    b: B,
    d: i32,
}

#[test]
fn test_def() {
    assert_eq!(
        C::inline(),
        "{ b: { c: number, a: number, b: number, }, d: number, }"
    );
}
