<p align="center">
  <a href="https://wasmer.io" target="_blank" rel="noopener noreferrer">
    <img width="400" src="https://raw.githubusercontent.com/wasmerio/wasmer/master/logo.png" alt="Wasmer logo">
  </a>
</p>

<p align="center">
  <a href="https://spectrum.chat/wasmer">
    <img src="https://withspectrum.github.io/badge/badge.svg" alt="Join the Wasmer Community"></a>
  <a href="https://github.com/wasmerio/wasmer/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/wasmerio/wasmer.svg" alt="License"></a>
</p>

# The Ruby extension to run WebAssembly

The goal of the project is to be able to run WebAssembly binaries from
Ruby directly. So much fun coming!

_Under Development, don't use it in product yet_.

## What is WebAssembly?

Quoting [the WebAssembly site](https://webassembly.org/):

> WebAssembly (abbreviated Wasm) is a binary instruction format for a
> stack-based virtual machine. Wasm is designed as a portable target
> for compilation of high-level languages like C/C++/Rust, enabling
> deployment on the web for client and server applications.

About speed:

> WebAssembly aims to execute at native speed by taking advantage of
> [common hardware
> capabilities](https://webassembly.org/docs/portability/#assumptions-for-efficient-execution)
> available on a wide range of platforms.

About safety:

> WebAssembly describes a memory-safe, sandboxed [execution
> environment](https://webassembly.org/docs/semantics/#linear-memory) […].

## Example

There is a toy program in `examples/simple.rs`, written in Rust (or
any other language that compiles to Wasm):

```rust
#[no_mangle]
pub extern fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

Once this program compiled to WebAssembly, we end up with a
`examples/simple.wasm` binary file.

Then, we can execute it in Ruby (!) with the `examples/simple.rb` file:

```rb
require "wasmer"

bytes = IO.read "simple.wasm", mode: "rb"
instance = Instance.new bytes
puts instance.exports.sum 1, 2
```

And then, finally, enjoy by running:

```sh
$ ruby simple.rb
3
```

## API documentation

### The `Instance` class

Instantiates a WebAssembly module represented by bytes, and calls exported functions on it:

```ruby
require "wasmer"

# Get the Wasm module as bytes.
wasm_bytes = IO.read "my_program.wasm", mode: "rb"

# Instantiates the Wasm module.
instance = Instance.new wasm_bytes

# Call a function on it.
result = instance.exports.sum 1, 2

puts result # 3
```

All exported functions are accessible on the `exports`
getter. Arguments of these functions are automatically casted to
WebAssembly values.

The `memory` getter exposes the `Memory` class representing the memory
of that particular instance, e.g.:

```ruby
view = instance.memory.uint8_view
```

See below for more information.

### The `Memory` class

A WebAssembly instance has its own memory, represented by the `Memory`
class. It is accessible by the `Instance.memory` getter.

The `Memory` class offers methods to create views of the memory
internal buffer, e.g. `uint8_view`, `int8_view`, `uint16_view`
etc. All these methods accept one optional argument: `offset`, to
subset the memory buffer at a particular offset. These methods return
respectively a `*Array` object, i.e. `uint8_view` returns a
`Uint8Array` object etc.

```ruby
offset = 7
view = instance.memory.uint8_view offset

puts view[0]
```

#### The `*Array` classes

These classes represent views over a memory buffer of an instance.

| Class | View buffer as a sequence of… | Bytes per element |
|-|-|-|
| `Int8Array` | `int8` | 1 |
| `Uint8Array` | `uint8` | 1 |
| `Int16Array` | `int16` | 2 |
| `Uint16Array` | `uint16` | 2 |
| `Int32Array` | `int32` | 4 |
| `Uint32Array` | `uint32` | 4 |

All these classes share the same implementation. Taking the example of
`Uint8Array`, the class looks like this:

```ruby
class Uint8Array
    def length
    def [](index)
    def []=(index, value)
end
```

Let's see it in action:

```ruby
require "wasmer"

# Get the Wasm module as bytes.
wasm_bytes = IO.read "my_program.wasm", mode: "rb"

# Instantiates the Wasm module.
instance = Instance.new wasm_bytes

# Call a function that returns a pointer to a string for instance.
pointer = instance.exports.return_string

# Get the memory view, with the offset set to `pointer` (default is 0).
memory = instance.memory.uint8_view pointer

# Read the string pointed by the pointer.
nth = 0
string = ""

while true
  char = memory[nth]

  if 0 == char
    break
  end

  string += char.chr
  nth += 1
end

puts string # Hello, World!
```

Each view shares the same memory buffer internally.

### The `Module` class

The `Module` class contains one static method `validate`, that checks
whether the given bytes represent valid WebAssembly bytes:

```ruby
require "wasmer"

wasm_bytes = IO.read "my_program.wasm", mode: "rb"

if not Module.validate wasm_bytes
    puts "The program seems corrupted."
end
```

This function returns a boolean.

## Install and Testing

To compile the entire project, run the following commands:

```sh
$ just rust
$ just test
$ ruby examples/simple.rb
```

(Yes, you need [`just`](https://github.com/casey/just/)).

## License

The entire project is under the BSD-3-Clause license. Please read [the
`LICENSE` file][license].


[license]: https://github.com/wasmerio/wasmer/blob/master/LICENSE
