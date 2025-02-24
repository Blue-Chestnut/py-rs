#![allow(dead_code, clippy::disallowed_names)]

use std::{collections::BTreeSet, rc::Rc};

use chrono::NaiveDateTime;
use py_rs::PY;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, PY)]
#[py(rename_all = "lowercase")]
#[py(export, export_to = "UserRole.py")]
enum Role {
    User,
    #[py(rename = "administrator")]
    Admin,
}

#[derive(Serialize, PY)]
#[py(rename_all = "lowercase")]
#[py(export, export_to = "UserRole.py")]
struct Person {
    role: Role,
}

#[derive(Serialize, PY)]
// when 'serde-compat' is enabled, py-rs tries to use supported serde attributes.
#[serde(rename_all = "UPPERCASE")]
#[py(export)]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, PY)]
#[py(export)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
    role: Role,
    family: Vec<User>,
    #[py(inline)]
    gender: Gender,
    token: Uuid,
    #[py(type = "string")]
    created_at: NaiveDateTime,
}

#[derive(Serialize, PY)]
#[serde(tag = "type", rename_all = "snake_case")]
#[py(export)]
enum Vehicle {
    Bicycle { color: String },
    Car { brand: String, color: String },
}

#[derive(Serialize, PY)]
#[py(export)]
struct Point<T>
where
    T: PY,
{
    time: u64,
    value: T,
}

#[derive(Serialize, PY)]
#[serde(default)]
#[py(export)]
struct Series {
    points: Vec<Point<u64>>,
}

#[derive(Serialize, PY)]
#[serde(tag = "kind", content = "d")]
#[py(export)]
enum SimpleEnum {
    A,
    B,
}

#[derive(Serialize, PY)]
// #[serde(tag = "kind", content = "data")]
#[py(export)]
enum ComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W((SimpleEnum, SimpleEnum)),
    F { nested: SimpleEnum },
    V(Vec<Series>),
    U(Box<User>),
}

#[derive(Serialize, PY)]
#[serde(tag = "kind")]
#[py(export)]
enum InlineComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W(SimpleEnum),
    F { nested: SimpleEnum },
    V(Vec<Series>),
    U(Box<User>),
}

/// This is a complex struct
#[derive(Serialize, PY)]
#[serde(rename_all = "camelCase")]
#[py(export)]
struct ComplexStruct {
    /// This is a field
    #[serde(default)]
    pub string_tree: Option<Rc<BTreeSet<String>>>,
}
