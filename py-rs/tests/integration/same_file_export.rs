use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "same_file_export/")]
struct DepA {
    foo: i32,
}

#[derive(PY)]
#[py(export, export_to = "same_file_export/")]
struct DepB {
    foo: i32,
}

#[derive(PY)]
#[py(export, export_to = "same_file_export/types.ts")]
struct A {
    foo: DepA,
}

#[derive(PY)]
#[py(export, export_to = "same_file_export/types.ts")]
struct B {
    foo: DepB,
}

#[derive(PY)]
#[py(export, export_to = "same_file_export/types.ts")]
struct C {
    foo: DepA,
    bar: DepB,
    biz: B,
}
