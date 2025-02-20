use py_rs::PY;

// serde_json serializes this to `null`, so it's PY type is `null` as well.
#[derive(PY)]
#[py(export, export_to = "unit/")]
struct Unit;

// serde_json serializes this to `{}`.
// The PY type best describing an empty object is `Record<string, never>`.
#[derive(PY)]
#[py(export, export_to = "unit/")]
struct Unit2 {}

// serde_json serializes this to `[]`.
// The PY type best describing an empty array is `never[]`.
#[derive(PY)]
#[py(export, export_to = "unit/")]
struct Unit3();

// serde_json serializes this to `null`, so it's PY type is `null` as well.
#[derive(PY)]
#[py(export, export_to = "unit/")]
struct Unit4(());

#[test]
fn test() {
    assert_eq!("type Unit = null;", Unit::decl());
    assert_eq!("type Unit2 = Record<string, never>;", Unit2::decl());
    assert_eq!("type Unit3 = never[];", Unit3::decl());
    assert_eq!("type Unit4 = null;", Unit4::decl());
}
