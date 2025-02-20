use py_rs::PY;

#[derive(PY)]
#[py(as = "T")]
pub enum UntaggedEnum<T: PY> {
    Left(T),
    Right(T),
}

#[test]
pub fn top_level_type_as_enum() {
    assert_eq!(UntaggedEnum::<String>::inline(), r#"string"#)
}

#[derive(PY)]
#[py(as = "T")]
pub struct Wrapper<T: PY>(T);

#[test]
pub fn top_level_type_as_struct() {
    assert_eq!(Wrapper::<String>::inline(), r#"string"#)
}
