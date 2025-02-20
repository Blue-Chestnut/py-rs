#![allow(dead_code)]
use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "list/")]
struct List {
    data: Option<Vec<u32>>,
}

#[test]
fn list() {
    assert_eq!(List::decl(), "type List = { data: Array<number> | null, };");
}
