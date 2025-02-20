#![allow(dead_code)]

use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "union_rename/")]
#[py(rename_all = "lowercase", rename = "SimpleEnum")]
enum RenamedEnum {
    #[py(rename = "ASDF")]
    A,
    B,
    C,
}

#[test]
fn test_simple_enum() {
    assert_eq!(
        RenamedEnum::decl(),
        r#"type SimpleEnum = "ASDF" | "b" | "c";"#
    )
}
