use tokio::sync::{Mutex, OnceCell, RwLock};

use super::{impl_wrapper, TypeVisitor, PY};

impl_wrapper!(impl<T: PY> PY for Mutex<T>);
impl_wrapper!(impl<T: PY> PY for OnceCell<T>);
impl_wrapper!(impl<T: PY> PY for RwLock<T>);
