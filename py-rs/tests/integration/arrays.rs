#![allow(dead_code)]

use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "arrays/")]
struct Interface {
    a: [i32; 4],
}

#[test]
fn free() {
    assert_eq!(<[String; 4]>::inline(), "[string, string, string, string]")
}

#[test]
fn interface() {
    assert_eq!(
        Interface::inline(),
        "{ a: [number, number, number, number], }"
    )
}

#[test]
fn newtype() {
    #[derive(PY)]
    struct Newtype(#[allow(dead_code)] [i32; 4]);

    assert_eq!(Newtype::inline(), "[number, number, number, number]")
}
