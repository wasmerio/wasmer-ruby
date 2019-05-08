//! Functions to handle error or exception correctly.

use rutie::{AnyException, VM};

pub fn unwrap_or_raise<Output, Function>(f: Function) -> Output
where
    Function: FnOnce() -> Result<Output, AnyException>,
{
    match f() {
        Ok(x) => x,
        Err(e) => {
            VM::raise_ex(e);
            unreachable!()
        }
    }
}
