$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)

require "wasmer"

file = File.expand_path "simple.wasm", File.dirname(__FILE__)

# Let's define the store, that holds the engine, that holds the compiler.
store = Wasmer::Store.new

# Let's compile the module to be able to execute it!
module_ = Wasmer::Module.new store, IO.read(file, mode: "rb")

# Now the module is compiled, we can instantiate it.
instance = Wasmer::Instance.new module_, nil

# Call the exported `sum` function.
result = instance.exports.sum.(5, 37)

puts result # 42!
