//! The `ExportedFunctions` WebAssembly class.

use crate::error::unwrap_or_raise;
use lazy_static::lazy_static;
use rutie::{
    class, methods,
    rubysys::{class, value::ValueType},
    types::{Argc, Value},
    util::str_to_cstring,
    wrappable_struct, AnyException, AnyObject, Array, Boolean, Exception, Fixnum, Float, NilClass,
    Object, Symbol,
};
use std::rc::Rc;
use wasmer_runtime::{self as runtime, types::Type, DynFunc};

/// The `ExportedFunctions` Ruby class.
pub struct ExportedFunctions {
    /// The WebAssembly runtime.
    instance: Rc<runtime::Instance>,
}

impl ExportedFunctions {
    /// Create a new instance of the `ExportedFunctions` Ruby class.
    pub fn new(instance: Rc<runtime::Instance>) -> Self {
        Self { instance }
    }

    /// Check that an exported function exists.
    pub fn respond_to_missing(&self, method_name: &str) -> bool {
        self.instance.exports.get::<DynFunc>(method_name).is_ok()
    }

    /// Call an exported function on the given WebAssembly instance.
    pub fn method_missing(
        &self,
        method_name: &str,
        arguments: Array,
    ) -> Result<AnyObject, AnyException> {
        let function = self.instance.exports.get::<DynFunc>(method_name).map_err(|_| {
            AnyException::new(
                "RuntimeError",
                Some(&format!("Function `{}` does not exist.", method_name)),
            )
        })?;
        let signature = function.signature();
        let parameters = signature.params();
        let number_of_parameters = parameters.len() as isize;
        let number_of_arguments = arguments.length() as isize;
        let diff: isize = number_of_parameters - number_of_arguments;

        if diff > 0 {
            return Err(AnyException::new(
                "ArgumentError",
                Some(&format!(
                    "Missing {} argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                    diff, method_name, number_of_parameters, number_of_arguments
                )),
            ));
        } else if diff < 0 {
            return Err(AnyException::new(
                "ArgumentError",
                Some(&format!(
                    "Given {} extra argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                    diff.abs(), method_name, number_of_parameters, number_of_arguments
                )),
            ));
        }

        let mut function_arguments =
            Vec::<runtime::Value>::with_capacity(number_of_parameters as usize);

        for (nth, (parameter, argument)) in parameters.iter().zip(arguments.into_iter()).enumerate()
        {
            let value = match (parameter, argument.ty()) {
                (Type::I32, ValueType::Fixnum) => runtime::Value::I32(
                    argument
                        .try_convert_to::<Fixnum>()
                        .map_err(|_| {
                            AnyException::new(
                                "TypeError",
                                Some(&format!(
                                    "Cannot convert argument #{} to a WebAssembly i32 value.",
                                    nth + 1
                                )),
                            )
                        })?
                        .to_i32(),
                ),
                (Type::I64, ValueType::Fixnum) => runtime::Value::I64(
                    argument
                        .try_convert_to::<Fixnum>()
                        .map_err(|_| {
                            AnyException::new(
                                "TypeError",
                                Some(&format!(
                                    "Cannot convert argument #{} to a WebAssembly i64 value.",
                                    nth + 1
                                )),
                            )
                        })?
                        .to_i64(),
                ),
                (Type::F32, ValueType::Float) => runtime::Value::F32(
                    argument
                        .try_convert_to::<Float>()
                        .map_err(|_| {
                            AnyException::new(
                                "TypeError",
                                Some(&format!(
                                    "Cannot convert argument #{} to a WebAssembly f32 value.",
                                    nth + 1
                                )),
                            )
                        })?
                        .to_f64() as f32,
                ),
                (Type::F64, ValueType::Float) => runtime::Value::F64(
                    argument
                        .try_convert_to::<Float>()
                        .map_err(|_| {
                            AnyException::new(
                                "TypeError",
                                Some(&format!(
                                    "Cannot convert argument #{} to a WebAssembly f64 value.",
                                    nth + 1
                                )),
                            )
                        })?
                        .to_f64(),
                ),
                (_, ty) => {
                    return Err(AnyException::new(
                        "ArgumentError",
                        Some(&format!(
                            "Cannot convert argument #{} to a WebAssembly value. Only integers and floats are supported. Given `{:?}`.",
                            nth + 1,
                            ty
                        ))));
                }
            };

            function_arguments.push(value);
        }

        let results = function
            .call(function_arguments.as_slice())
            .map_err(|e| AnyException::new("RuntimeError", Some(&format!("{}", e))))?;

        if !results.is_empty() {
            Ok(match results[0] {
                runtime::Value::I32(result) => Fixnum::new(result as i64).into(),
                runtime::Value::I64(result) => Fixnum::new(result).into(),
                runtime::Value::F32(result) => Float::new(result as f64).into(),
                runtime::Value::F64(result) => Float::new(result).into(),
                runtime::Value::V128(_result) => {
                    return Err(AnyException::new(
                        "RuntimeError",
                        Some("Type `V128` isn't supported yet."),
                    ))
                }
            })
        } else {
            Ok(NilClass::new().into())
        }
    }
}

wrappable_struct!(
    ExportedFunctions,
    ExportedFunctionsWrapper,
    EXPORTED_FUNCTIONS_WRAPPER
);

class!(RubyExportedFunctions);

#[rustfmt::skip]
methods!(
    RubyExportedFunctions,
    itself,

    // Glue code to call the `ExportedFunctions.respond_to` method.
    fn ruby_exported_functions_method_exists(symbol: Symbol, _include_private: Boolean) -> Boolean {
        unwrap_or_raise(|| {
            let symbol = symbol?;
            let exported_functions = itself.get_data(&*EXPORTED_FUNCTIONS_WRAPPER);

            Ok(Boolean::new(exported_functions.respond_to_missing(symbol.to_str())))
        })
    }
);

#[allow(improper_ctypes_definitions)]
/// Glue code to call the `ExportedFunctions.method_missing` method.
pub extern "C" fn ruby_exported_functions_method_missing(
    argc: Argc,
    argv: *const AnyObject,
    itself: RubyExportedFunctions,
) -> AnyObject {
    unwrap_or_raise(|| {
        let arguments = Value::from(0);

        unsafe {
            let argv_pointer = argv as *const Value;

            class::rb_scan_args(argc, argv_pointer, str_to_cstring("*").as_ptr(), &arguments)
        };

        let mut arguments = Array::from(arguments);
        let method_name = unsafe { arguments.shift().to::<Symbol>() };
        let method_name = method_name.to_str();

        itself
            .get_data(&*EXPORTED_FUNCTIONS_WRAPPER)
            .method_missing(method_name, arguments)
    })
}
