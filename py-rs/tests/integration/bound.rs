#![allow(dead_code)]

use py_rs::PY;

trait Driver {
    type Info;
}

struct TsDriver;

#[derive(PY)]
struct TsInfo;

impl Driver for TsDriver {
    type Info = TsInfo;
}

#[derive(PY)]
#[py(export, export_to = "bound/")]
#[py(concrete(D = TsDriver))]
struct Inner<D: Driver> {
    info: D::Info,
}

#[derive(PY)]
#[py(export, export_to = "bound/")]
#[py(concrete(D = TsDriver), bound = "D::Info: PY")]
struct Outer<D: Driver> {
    inner: Inner<D>,
}

#[test]
fn test_bound() {
    assert_eq!(Outer::<TsDriver>::decl(), "type Outer = { inner: Inner, };");
    assert_eq!(Inner::<TsDriver>::decl(), "type Inner = { info: TsInfo, };");
}
