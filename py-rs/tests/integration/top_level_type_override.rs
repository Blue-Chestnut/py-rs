use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "top_level_type_override/")]
#[py(type = "string")]
#[non_exhaustive]
pub enum IncompleteEnum {
    Foo,
    Bar,
    Baz,
    // more
}

#[test]
pub fn top_level_type_override_enum() {
    assert_eq!(IncompleteEnum::inline(), r#"string"#)
}

#[derive(PY)]
#[py(export, export_to = "top_level_type_override/")]
#[py(type = "string")]
pub struct DataUrl {
    pub mime: String,
    pub contents: Vec<u8>,
}

#[test]
pub fn top_level_type_override_struct() {
    assert_eq!(DataUrl::inline(), r#"string"#)
}
