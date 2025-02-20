#![allow(dead_code)]

#[derive(py_rs::PY)]
struct Test<T> {
    field: T,
}
