#![allow(dead_code)]
use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "path_bug/aaa/")]
struct Foo {
    bar: Bar,
}

#[derive(PY)]
#[py(export_to = "../bindings/path_bug/")]
struct Bar {
    i: i32,
}

#[test]
fn path_bug() {
    export_bindings_foo();

    assert!(Foo::default_output_path().unwrap().is_file());
    assert!(Bar::default_output_path().unwrap().is_file());
}
