# coding: utf-8
require "./prelude"

# A Wasm module can import entities, like functions, memories,
# globals and tables.
#
# This example illustrates how to use imported functions, aka host
# functions.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ ruby examples/imports_function.rb
# ```
#
# Ready?

# Let's declare the Wasm module with the text representation.
# If this module was written in Rust, it would have been:
#
# ```rs
# extern "C" {
#     fn sum(x: i32, y: i32) -> i32;
# }
# 
# #[no_mangle]
# pub extern "C" fn add_one(x: i32) -> i32 {
#     unsafe { sum(x, 1) }
# }
# ```
wasm_bytes = Wasmer::wat2wasm(
  (<<~WAST)
  (module
    (import "env" "sum" (func $sum (param i32 i32) (result i32)))
    (func (export "add_one") (param $x i32) (result i32)
      local.get $x
      i32.const 1
      call $sum))
  WAST
)

# Create a store.
store = Wasmer::Store.new

# Let's compile the Wasm module.
module_ = Wasmer::Module.new store, wasm_bytes

# Here we go.
#
# When creating an `Instance`, we can pass an `ImportObject`. All
# entities that must be imported are registered inside the
# `ImportObject`.
import_object = Wasmer::ImportObject.new

# Let's write the Ruby function that is going to be imported,
# i.e. called by the WebAssembly module.
def sum(x, y)
  x + y
end

sum_host_function = Wasmer::Function.new(
  store,
  method(:sum),
  #                         x                  y                    result
  Wasmer::FunctionType.new([Wasmer::Type::I32, Wasmer::Type::I32], [Wasmer::Type::I32])
)

# Now let's register the `sum` import inside the `env` namespace.
import_object.register(
  "env",
  {
    :sum => sum_host_function,
  }
)

# Let's instantiate the module!
instance = Wasmer::Instance.new module_, import_object

# And finally, call the `add_one` exported function!
assert { instance.exports.add_one.(41) == 42 }
