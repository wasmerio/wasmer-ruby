//! Functions to handle error or exception correctly.

use rutie::{AnyException, VM};

pub fn unwrap_or_raise<T, F: FnOnce() -> Result<T, AnyException>>(f: F) -> T {
    let result = f();
    match result {
        Ok(x) => x,
        Err(e) => {
            VM::raise_ex(e);
            unreachable!()
        }
    }
}
