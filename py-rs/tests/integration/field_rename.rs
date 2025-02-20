#![allow(dead_code)]

use py_rs::PY;

#[derive(PY)]
struct Rename {
    a: i32,
    #[py(rename = "bb")]
    b: i32,
}

#[test]
fn test() {
    assert_eq!(Rename::inline(), "{ a: number, bb: number, }")
}
