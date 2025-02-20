#![allow(dead_code)]

use std::collections::{BTreeSet, HashSet};

use py_rs::PY;

#[derive(PY, Eq, PartialEq, Hash)]
#[py(export, export_to = "hashset/")]
struct CustomValue;

#[derive(PY)]
#[py(export, export_to = "hashset/")]
struct HashSetWithCustomType {
    set: HashSet<CustomValue>,
}

#[derive(PY)]
#[py(export, export_to = "hashset/")]
struct BTreeSetWithCustomType {
    set: BTreeSet<CustomValue>,
}

#[test]
fn with_custom_types() {
    assert_eq!(
        HashSetWithCustomType::inline(),
        BTreeSetWithCustomType::inline()
    );
    assert_eq!(
        HashSetWithCustomType::decl(),
        "type HashSetWithCustomType = { set: Array<CustomValue>, };"
    );
}
