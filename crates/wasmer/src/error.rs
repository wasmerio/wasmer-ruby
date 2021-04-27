//! Functions to handle error or exception correctly.

use rutie::{AnyException, Exception, VM};

#[allow(unused)]
pub(crate) fn unwrap_or_raise<Output, Function>(f: Function) -> Output
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

pub trait ErrorType {
    fn name() -> &'static str;
}

macro_rules! declare_error {
    ($name:ident) => {
        pub struct $name;

        impl ErrorType for $name {
            fn name() -> &'static str {
                stringify!($name)
            }
        }
    };

    ( $( $name:ident ),+ $(,)? ) => {
        $( declare_error!($name); )*
    }
}

declare_error!(RuntimeError, TypeError);

pub fn to_ruby_err<Type, Error>(error: Error) -> AnyException
where
    Type: ErrorType,
    Error: ToString,
{
    AnyException::new(Type::name(), Some(error.to_string().as_ref()))
}
