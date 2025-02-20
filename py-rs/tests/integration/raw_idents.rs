#![allow(non_camel_case_types, dead_code)]

use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "raw_idents/")]
struct r#struct {
    r#type: i32,
    r#use: i32,
    r#struct: i32,
    r#let: i32,
    r#enum: i32,
}

#[test]
fn raw_idenpy() {
    let out = <r#struct as PY>::decl();
    assert_eq!(
        out,
        "type struct = { type: number, use: number, struct: number, let: number, enum: number, };"
    );
}
