//#![deny(warnings)]

use lazy_static::lazy_static;
use rutie::{
    class, methods,
    rubysys::{class, value::ValueType},
    types::{Argc, Value},
    util::str_to_cstring,
    wrappable_struct, AnyObject, Array, Class, Fixnum, Float, Object, Symbol,
};
use std::{mem, rc::Rc};
use wasmer_runtime::{self as runtime, imports};
use wasmer_runtime_core::types::Type;

static WASM: &'static [u8] = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x06, 0x01, 0x60, 0x01, 0x7f, 0x01, 0x7f,
    0x03, 0x02, 0x01, 0x00, 0x07, 0x0b, 0x01, 0x07, 0x61, 0x64, 0x64, 0x5f, 0x6f, 0x6e, 0x65, 0x00,
    0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x41, 0x01, 0x6a, 0x0b, 0x00, 0x1a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x01, 0x0a, 0x01, 0x00, 0x07, 0x61, 0x64, 0x64, 0x5f, 0x6f, 0x6e, 0x65, 0x02,
    0x07, 0x01, 0x00, 0x01, 0x00, 0x02, 0x70, 0x30,
];

pub struct ExportedFunctions {
    instance: Rc<runtime::Instance>,
}

impl ExportedFunctions {
    pub fn new(instance: Rc<runtime::Instance>) -> Self {
        Self { instance }
    }

    pub fn method_missing(&self, method_name: &str, arguments: Array) -> AnyObject {
        let function = self.instance.dyn_func(method_name).unwrap();
        let signature = function.signature();
        let parameters = signature.params();
        let number_of_parameters = parameters.len() as isize;
        let number_of_arguments = arguments.length() as isize;
        let diff: isize = number_of_parameters - number_of_arguments;

        if diff > 0 {
            panic!("Missing arguments");
        } else if diff < 0 {
            panic!("Too much arguments");
        }

        let mut function_arguments =
            Vec::<runtime::Value>::with_capacity(number_of_parameters as usize);

        for (parameter, argument) in parameters.iter().zip(arguments.into_iter()) {
            let value = match (parameter, argument.ty()) {
                (Type::I32, ValueType::Fixnum) => {
                    runtime::Value::I32(argument.try_convert_to::<Fixnum>().unwrap().to_i32())
                }
                (Type::I64, ValueType::Fixnum) => {
                    runtime::Value::I64(argument.try_convert_to::<Fixnum>().unwrap().to_i64())
                }
                (Type::F32, ValueType::Float) => {
                    runtime::Value::F32(argument.try_convert_to::<Float>().unwrap().to_f64() as f32)
                }
                (Type::F64, ValueType::Float) => {
                    runtime::Value::F64(argument.try_convert_to::<Float>().unwrap().to_f64())
                }
                _ => panic!("aaahhh"),
            };

            function_arguments.push(value);
        }

        let results = function.call(function_arguments.as_slice()).unwrap();

        match results[0] {
            runtime::Value::I32(result) => Fixnum::new(result as i64).into(),
            runtime::Value::I64(result) => Fixnum::new(result).into(),
            runtime::Value::F32(result) => Float::new(result as f64).into(),
            runtime::Value::F64(result) => Float::new(result).into(),
        }
    }
}

wrappable_struct!(
    ExportedFunctions,
    ExportedFunctionsWrapper,
    EXPORTED_FUNCTIONS_WRAPPER
);

class!(RubyExportedFunctions);

pub extern "C" fn ruby_exported_functions_method_missing(
    argc: Argc,
    argv: *const AnyObject,
    itself: RubyExportedFunctions,
) -> AnyObject {
    let arguments = Value::from(0);

    unsafe {
        let argv_pointer: *const Value = mem::transmute(argv);

        class::rb_scan_args(argc, argv_pointer, str_to_cstring("*").as_ptr(), &arguments)
    };

    let mut arguments = Array::from(arguments);
    let method_name = unsafe { arguments.shift().to::<Symbol>() };
    let method_name = method_name.to_str();

    itself
        .get_data(&*EXPORTED_FUNCTIONS_WRAPPER)
        .method_missing(method_name, arguments)
}

pub struct Instance {
    instance: Rc<runtime::Instance>,
}

impl Instance {
    pub fn new() -> Self {
        let import_object = imports! {};
        let instance = Rc::new(runtime::instantiate(WASM, &import_object).unwrap());

        Self { instance }
    }
}

wrappable_struct!(Instance, InstanceWrapper, INSTANCE_WRAPPER);

class!(RubyInstance);

#[rustfmt::skip]
methods!(
    RubyInstance,
    _itself,

    fn ruby_instance_new() -> AnyObject {
        let instance = Instance::new();
        let exported_functions = ExportedFunctions::new(instance.instance.clone());

        let mut ruby_instance: AnyObject = Class::from_existing("Instance").wrap_data(instance, &*INSTANCE_WRAPPER);
        let ruby_exported_functions: RubyExportedFunctions = Class::from_existing("ExportedFunctions").wrap_data(exported_functions, &*EXPORTED_FUNCTIONS_WRAPPER);

        ruby_instance.instance_variable_set("@exports", ruby_exported_functions);

        ruby_instance
    }

    fn ruby_instance_exported_functions() -> RubyExportedFunctions {
        unsafe {
            _itself.instance_variable_get("@exports").to::<RubyExportedFunctions>()
        }
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_wasmer() {
    let instance_data_class = Class::from_existing("Object");
    let exported_functions_data_class = Class::from_existing("Object");

    Class::new("Instance", Some(&instance_data_class)).define(|itself| {
        itself.def_self("new", ruby_instance_new);
        itself.def("exports", ruby_instance_exported_functions);
    });

    Class::new("ExportedFunctions", Some(&exported_functions_data_class)).define(|itself| {
        itself.def("method_missing", ruby_exported_functions_method_missing);
    });
}
