#![allow(unused)]

use ::py_rs::PY;

mod ts_rs {}

#[derive(PY)]
struct Foo {
    x: u32,
}
