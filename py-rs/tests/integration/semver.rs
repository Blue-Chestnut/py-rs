#![allow(dead_code)]
#![cfg(feature = "semver-impl")]

use semver::Version;
use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "semver/")]
struct Semver {
    version: Version,
}

#[test]
fn semver() {
    assert_eq!(Semver::decl(), "type Semver = { version: string, };")
}
