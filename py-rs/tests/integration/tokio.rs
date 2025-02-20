#![cfg(feature = "tokio-impl")]

use tokio::sync::{Mutex, OnceCell, RwLock};
use py_rs::PY;

#[derive(PY)]
#[py(export, export_to = "tokio/")]
#[py(concrete(T = i32))]
struct Tokio<T: 'static> {
    mutex: Mutex<T>,
    once_cell: OnceCell<T>,
    rw_lock: RwLock<T>,
}

#[test]
fn tokio() {
    assert_eq!(
        Tokio::<String>::decl(),
        "type Tokio = { mutex: number, once_cell: number, rw_lock: number, };"
    )
}
