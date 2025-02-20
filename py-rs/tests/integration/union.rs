#![allow(dead_code)]

use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "union/")]
enum SimpleEnum {
    #[py(rename = "asdf")]
    A,
    B,
    C,
}

#[test]
fn test_empty() {
    #[derive(PY)]
    enum Empty {}

    assert_eq!(Empty::decl(), "type Empty = never;")
}

#[test]
fn test_simple_enum() {
    assert_eq!(
        SimpleEnum::decl(),
        r#"type SimpleEnum = "asdf" | "B" | "C";"#
    )
}
