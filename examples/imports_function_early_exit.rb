# coding: utf-8
require "./prelude"

# A Wasm module can import entities, like functions, memories,
# globals and tables.
#
# This example illustrates how to use an imported function that fails!
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ rb examples/imports_function_early_exit.rb
# ```
#
# Ready?

# Let's declare the Wasm module with the text representation.
wasm_bytes = Wasmer::wat2wasm(
  (<<~WAST)
  (module
    (type $run_t (func (param i32 i32) (result i32)))
    (type $early_exit_t (func (param) (result)))
    (import "env" "early_exit" (func $early_exit (type $early_exit_t)))
    (func $run (type $run_t) (param $x i32) (param $y i32) (result i32)
      (call $early_exit)
      (i32.add
          local.get $x
          local.get $y))
    (export "run" (func $run)))
  WAST
)

# Create a store.
store = Wasmer::Store.new

# Let's compile the Wasm module.
module_ = Wasmer::Module.new store, wasm_bytes

# Here we go.
#
# Let's write the Ruby function that is going to… fail!
def early_exit
  raise "oops"
end

# When creating an `Instance`, we can pass an `ImportObject`. All
# entities that must be imported are registered inside the
# `ImportObject`.
import_object = Wasmer::ImportObject.new

# Now let's register the `sum` import inside the `env` namespace.
import_object.register(
  "env",
  {
    :early_exit => Wasmer::Function.new(
      store,
      method(:early_exit),
      Wasmer::FunctionType.new([], [])
    ),
  }
)

# Let's instantiate the module!
instance = Wasmer::Instance.new module_, import_object

# And finally, call the `run` exported function!
begin
  instance.exports.run.(1, 2)
rescue RuntimeError => e
  assert { e.message == "oops" }
else
  assert { false }
end
