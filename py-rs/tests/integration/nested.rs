#![allow(dead_code)]

use std::{cell::Cell, rc::Rc, sync::Arc};

use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "nested/")]
struct A {
    x1: Arc<i32>,
    y1: Cell<i32>,
}

#[derive(PY)]
#[py(export, export_to = "nested/")]
struct B {
    a1: Box<A>,
    #[py(inline)]
    a2: A,
}

#[derive(PY)]
#[py(export, export_to = "nested/")]
struct C {
    b1: Rc<B>,
    #[py(inline)]
    b2: B,
}

#[test]
fn test_nested() {
    assert_eq!(
        C::inline(),
        "{ b1: B, b2: { a1: A, a2: { x1: number, y1: number, }, }, }"
    );
}
