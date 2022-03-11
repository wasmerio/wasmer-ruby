require_relative "prelude"

# A Wasm module can export entities, like functions, memories,
# globals and tables.
#
# This example illustrates how to use exported memories.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ ruby examples/exports_memory.rb
# ```
#
# Ready?

# Let's declare the Wasm module with the text representation.
# If this module was written in Rust, it would have been:
#
# ```rs
# #[no_mangle]
# pub extern fn hello() -> *const u8 {
#     b"Hello, World!\0".as_ptr()
# }
# ```
wasm_bytes = Wasmer::wat2wasm(
  (<<~WAST)
  (module
    (type $hello_t (func (result i32)))
    (func $hello (type $hello_t) (result i32)
        i32.const 42)
    (memory $memory 1)
    (export "hello" (func $hello))
    (export "mem" (memory $memory))
    (data (i32.const 42) "Hello, World!"))
  WAST
)

# Create a store.
store = Wasmer::Store.new

# Let's compile the Wasm module.
module_ = Wasmer::Module.new store, wasm_bytes

# Let's instantiate the Wasm module.
instance = Wasmer::Instance.new module_, nil

# OK, here go. First, let's call `hello`. It returns a pointer to the
# string in memory.
pointer = instance.exports.hello.()

# Since the pointer is a constant here, it's easy to assert its value.
assert { pointer == 42 }

# Now let's read the string. It lives in memory. Usually the main
# memory is named `memory`, but the sake of not being simple, the
# memory is named `mem` in our case.
memory = instance.exports.mem

# See, it's a `Memory`!
assert { memory.is_a?(Wasmer::Memory) }

# Next, read it. We have multiple options thanks to the views
# (`Uint8Array`, `Int8Array`, `Uint16Array` etc.). Let's get a `uint8`
# view starting from `pointer`.
reader = memory.uint8_view pointer

# Go read. We know `Hello, World!` is 13 bytes long.
#
# Don't forget that we read bytes. We need to decode them!
returned_string = reader.take(13).pack("C*").force_encoding('utf-8')

assert { returned_string == 'Hello, World!' }

# Yeah B-)!
