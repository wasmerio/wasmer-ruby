<p align="center">
  <a href="https://wasmer.io" target="_blank" rel="noopener noreferrer">
    <img width="400" src="https://raw.githubusercontent.com/wasmerio/wasmer/master/logo.png" alt="Wasmer logo">
  </a>
</p>

<p align="center">
  <a href="https://spectrum.chat/wasmer">
    <img src="https://withspectrum.github.io/badge/badge.svg" alt="Join the Wasmer Community">
  </a>
  <a href="https://github.com/wasmerio/wasmer/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/wasmerio/wasmer.svg" alt="License">
  </a>
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

## Usage and Testing

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
