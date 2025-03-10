#![allow(dead_code)]

use std::{borrow::Cow, rc::Rc, sync::Arc};

use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "unsized/")]
struct S<'a> {
    b: Box<str>,
    c: Cow<'a, str>,
    r: Rc<str>,
    a: Arc<str>,
}

#[test]
fn contains_str() {
    assert_eq!(
        S::decl(),
        "type S = { b: string, c: string, r: string, a: string, };"
    )
}
