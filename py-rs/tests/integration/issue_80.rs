use serde::Serialize;
use py_rs::PY;

#[derive(PY, Serialize)]
#[py(export, export_to = "issue_80/")]
pub enum SomeTypeList {
    Value1 {
        #[serde(skip)]
        #[py(skip)]
        skip_this: String,
    },
    Value2,
}

#[test]
fn issue_80() {
    let ty = SomeTypeList::inline();
    assert_eq!(ty, r#"{ "Value1": {  } } | "Value2""#);
}
