#![cfg(feature = "serde-compat")]
#![allow(dead_code)]

// from issue #107. This does now no longer generate a warning.

use serde::{Deserialize, Serialize};
use py_rs::PY;

fn default_http_version() -> String {
    "2".to_owned()
}

#[derive(Debug, Clone, Deserialize, Serialize, PY)]
#[py(export, export_to = "serde_skip_with_default/")]
pub struct Foobar {
    // #[py(skip)]
    #[serde(skip, default = "default_http_version")]
    pub http_version: String,
    pub something_else: i32,
}

#[test]
fn serde_skip_with_default() {
    assert_eq!(Foobar::decl(), "type Foobar = { something_else: number, };");
}
