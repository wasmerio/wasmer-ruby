use crate::{
    error::{to_ruby_err, TypeError},
    prelude::*,
    store::Store,
    types::FunctionType,
};
use rutie::{AnyObject, Object, Proc, Symbol};
use std::sync::Arc;

#[derive(Clone)]
struct Callable(Arc<dyn Fn(&[AnyObject]) -> AnyObject>);

unsafe impl Send for Callable {}
unsafe impl Sync for Callable {}

#[rubyclass(module = "Wasmer")]
pub struct Function {
    inner: wasmer::Function,
}

impl Function {
    pub fn raw_new(inner: wasmer::Function) -> Self {
        Self { inner }
    }

    fn inner(&self) -> &wasmer::Function {
        &self.inner
    }
}

#[rubymethods]
impl Function {
    pub fn new(
        store: &Store,
        function: &AnyObject,
        function_type: &FunctionType,
    ) -> RubyResult<AnyObject> {
        let function = Callable(if let Ok(symbol) = function.try_convert_to::<Symbol>() {
            Arc::new(move |arguments| symbol.to_proc().call(arguments))
        } else if let Ok(proc) = function.try_convert_to::<Proc>() {
            Arc::new(move |arguments| proc.call(arguments))
        } else if rutie::util::is_method(*function.as_ref()) {
            let function = function.clone();

            Arc::new(move |arguments| unsafe { function.send("call", arguments) })
        } else {
            return Err(to_ruby_err::<TypeError, _>(
                "Argument #1 of `Function.new` must be either a `Symbol`, a `Proc`, or a `Method`",
            ));
        });

        #[derive(wasmer::WasmerEnv, Clone)]
        struct Environment {
            ruby_callable: Callable,
        }

        let environment = Environment {
            ruby_callable: function,
        };

        let function_type: wasmer::FunctionType = function_type.into();
        let host_function = wasmer::Function::new_with_env(
            store.inner(),
            function_type,
            environment,
            |environment,
             arguments: &[wasmer::Value]|
             -> Result<Vec<wasmer::Value>, wasmer::RuntimeError> {
                let ruby_callable = &environment.ruby_callable.0;
                let _ = ruby_callable(&[]);

                Ok(vec![])
            },
        );

        Ok(Function::ruby_new(Function {
            inner: host_function,
        }))
    }

    pub fn r#type(&self) -> RubyResult<AnyObject> {
        Ok(FunctionType::ruby_new(self.inner().ty().into()))
    }
}

pub(crate) mod ruby_function_extra {
    use crate::{
        error::{to_ruby_err, unwrap_or_raise, RubyResult, RuntimeError},
        values::{to_ruby_object, to_wasm_value},
    };
    use rutie::{
        rubysys::class,
        types::{Argc, Value},
        util::str_to_cstring,
        AnyObject, Array, NilClass, Object,
    };
    use rutie_derive::UpcastRubyClass;

    #[allow(improper_ctypes_definitions)] // No choice, that's how `rutie` is designed.
    pub extern "C" fn call(
        argc: Argc,
        argv: *const AnyObject,
        itself: super::RubyFunction,
    ) -> AnyObject {
        unwrap_or_raise(|| {
            let arguments = Value::from(0);

            unsafe {
                let argv_pointer = argv as *const Value;

                class::rb_scan_args(argc, argv_pointer, str_to_cstring("*").as_ptr(), &arguments)
            };

            let function = itself.upcast();
            let arguments: Vec<wasmer::Value> = Array::from(arguments)
                .into_iter()
                .zip(function.inner().ty().params().iter().cloned())
                .map(|(value, ty)| to_wasm_value((&value, ty)))
                .collect::<RubyResult<_>>()?;

            let results = function
                .inner()
                .call(&arguments)
                .map(<[_]>::into_vec)
                .map_err(to_ruby_err::<RuntimeError, _>)?;

            Ok(match results.len() {
                0 => NilClass::new().to_any_object(),
                1 => to_ruby_object(&results[0]),
                _ => results
                    .iter()
                    .map(to_ruby_object)
                    .collect::<Array>()
                    .to_any_object(),
            })
        })
    }
}
