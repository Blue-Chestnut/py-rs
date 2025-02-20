#![cfg(feature = "bson-uuid-impl")]

use bson::{oid::ObjectId, Uuid};
use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "bson/")]
struct User {
    _id: ObjectId,
    _uuid: Uuid,
}

#[test]
fn bson() {
    assert_eq!(User::decl(), "type User = { _id: string, _uuid: string, };")
}
