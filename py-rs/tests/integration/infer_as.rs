#![allow(dead_code)]

use py_rs::PY;

trait Bar {
    type Baz;
}

impl Bar for String {
    type Baz = i32;
}

#[derive(PY)]
#[py(export)]
struct Foo {
    #[py(optional, as = "Option<_>")]
    my_optional_bool: bool,

    #[py(as = "<_ as Bar>::Baz")]
    q_self: String,
}

#[test]
fn test() {
    assert_eq!(
        Foo::inline(),
        "{ my_optional_bool?: boolean, q_self: number, }"
    );
}
