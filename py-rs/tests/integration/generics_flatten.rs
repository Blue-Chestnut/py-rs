use py_rs_macros::PY;

// https://github.com/Aleph-Alpha/ts-rs/issues/335
#[derive(PY)]
#[py(export, export_to = "generics/flatten/")]
struct Item<D> {
    id: String,
    #[py(flatten)]
    inner: D,
}

#[derive(PY)]
#[py(export, export_to = "generics/flatten/")]
struct TwoParameters<A, B> {
    id: String,
    #[py(flatten)]
    a: A,
    #[py(flatten)]
    b: B,
    ab: (A, B),
}

#[derive(PY)]
#[py(export, export_to = "generics/flatten/")]
enum Enum<A, B> {
    A {
        #[py(flatten)]
        a: A,
    },
    B {
        #[py(flatten)]
        b: B,
    },
    AB(A, B),
}

#[test]
fn flattened_generic_parameters() {
    use py_rs::PY;

    #[derive(PY)]
    struct Inner {
        x: i32,
    }

    assert_eq!(Item::<()>::decl(), "type Item<D> = { id: string, } & D;");
    assert_eq!(
        TwoParameters::<(), ()>::decl(),
        "type TwoParameters<A, B> = { id: string, ab: [A, B], } & A & B;"
    );
    assert_eq!(
        Enum::<(), ()>::decl(),
        "type Enum<A, B> = { \"A\": A } | { \"B\": B } | { \"AB\": [A, B] };"
    );
}
