# coding: utf-8
require File.expand_path "../prelude", __FILE__

# Running a WASI compiled WebAssembly module with Wasmer.
#
# This example illustrates how to run WASI modules with
# Wasmer. To run WASI we have to have to do mainly 3 steps:
#
#   1. Create a `Wasi::Environment` instance,
#   2. Attach the imports from the `Wasi::Environment` to a new
#      instance,
#   3. Run the WASI module.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ ruby examples/wasi.rb
# ```
#
# Ready?

# Let's get the `wasi.wasm` bytes!
file = File.expand_path "appendices/wasi.wasm", File.dirname(__FILE__)
wasm_bytes = IO.read(file, mode: "rb")

# Create a store.
store = Wasmer::Store.new

# Let's compile the Wasm module, as usual.
module_ = Wasmer::Module.new store, wasm_bytes

# Here we go.
#
# First, let's extract the WASI version from the module. Why? Because
# WASI already exists in multiple versions, and it doesn't work the
# same way. So, to ensure compatibility, we need to know the version.
wasi_version = Wasmer::Wasi::get_version module_, true

# Second, create a `Wasi::Environment`. It contains everything related
# to WASI. To build such an environment, we must use the
# `Wasi::StateBuilder`.
#
# In this case, we specify the program name is `wasi_test_program`. We
# also specify the program is invoked with the `--test` argument, in
# addition to two environment variable: `COLOR` and
# `APP_SHOULD_LOG`. Finally, we map the `the_host_current_dir` to the
# current directory. There it is:
wasi_env =
  Wasmer::Wasi::StateBuilder.new('wasi_test_program')
    .argument('--test')
    .environment('COLOR', 'true')
    .environment('APP_SHOULD_LOG', 'false')
    .map_directory('the_host_current_dir', '.')
    .finalize

# From the WASI environment, we generate a custom import object. Why?
# Because WASI is, from the user perspective, a bunch of
# imports. Consequently `generate_import_object`… generates a
# pre-configured import object.
#
# Do you remember when we said WASI has multiple versions? Well, we
# need the WASI version here!
import_object = wasi_env.generate_import_object store, wasi_version

# Now we can instantiate the module.
instance = Wasmer::Instance.new module_, import_object

# The entry point for a WASI WebAssembly module is a function named
# `_start`. Let's call it and see what happens!
instance.exports._start.()

# It has printed:
#
# ```
# Found program name: `wasi_test_program`
# Found 1 arguments: --test
# Found 2 environment variables: COLOR=true, APP_SHOULD_LOG=false
# Found 1 preopened directories: DirEntry("/the_host_current_dir")
# ```
#
# on the standard output.
