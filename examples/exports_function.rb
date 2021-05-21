require File.expand_path "../prelude", __FILE__

# A Wasm module can export entities, like functions, memories,
# globals and tables.
#
# This example illustrates how to use exported functions.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ ruby examples/exports_function.rb
# ```
#
# Ready?

# Let's declare the Wasm module with the text representation.
wasm_bytes = Wasmer::wat2wasm(
  (<<~WAST)
  (module
    (type $sum_t (func (param i32 i32) (result i32)))
    (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
      local.get $x
      local.get $y
      i32.add)
    (export "sum" (func $sum_f)))
  WAST
)

# Create a store.
store = Wasmer::Store.new

# Let's compile the Wasm module.
module_ = Wasmer::Module.new store, wasm_bytes

# Let's instantiate the Wasm module.
instance = Wasmer::Instance.new module_, nil

# Here we go.
#
# An `Instance` has an `exports` getter, which returns the same
# `Exports` object (per `Instance`). `Exports.method_missing` is the
# only API to get an export. It will return either a `Function`, a
# `Memory`, a `Global` or a `Table`.
#
# Let's call the `sum` function with 1 and 2.
results = instance.exports.sum.(1, 2)

assert { results == 3 }

# But this is not always ideal. Keep in mind that a `Function` object
# is created everytime you call `Exports.method_missing`. Hence the
# following solution to store the function inside a variable.
sum = instance.exports.sum

assert { sum.is_a?(Wasmer::Function) }

# We use the `.(args)` notation to call the function. Calling `call`
# manually would have the same effect.
results = sum.(1, 2)
# or
results = sum.call(1, 2)

# Did you notice something? We didn't cast the Ruby values
# (arguments of `sum`) to WebAssembly values. It's done automatically!
#
# Same for the results. It's casted to Ruby values automatically.

assert { results == 3 }

# How cool is that :-)?
