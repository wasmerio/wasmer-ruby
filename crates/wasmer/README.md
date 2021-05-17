<div align="center">
  <a href="https://wasmer.io" target="_blank" rel="noopener noreferrer">
    <img width="300" src="https://raw.githubusercontent.com/wasmerio/wasmer/master/assets/logo.png" alt="Wasmer logo">
  </a>
  
  <h1>Wasmer Ruby</h1>
  
  <p>
    <a href="https://github.com/wasmerio/wasmer-ruby/actions?query=workflow%3A%22Build+and+Test%22">
      <img src="https://github.com/wasmerio/wasmer-ruby/workflows/Build%20and%20Test/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/wasmerio/wasmer-ruby/blob/master/LICENSE">
      <img src="https://img.shields.io/github/license/wasmerio/wasmer-ruby.svg" alt="License">
    </a>
    <a href="https://rubygems.org/gems/wasmer">
      <img src="https://img.shields.io/gem/v/wasmer.svg" alt="Wasmer on RubyGem">
    </a> 
    <a href="https://rubygems.org/gems/wasmer">
      <img src="https://img.shields.io/gem/dt/wasmer.svg" alt="Number of downloads">
    </a> 
    <a href="https://wasmerio.github.io/wasmer-ruby/wasmer_ruby/index.html">
      <img src="https://img.shields.io/badge/documentation-API-f06" alt="API Documentation">
    </a> 
  </p>

  <h3>
    <a href="https://wasmer.io/">Website</a>
    <span> • </span>
    <a href="https://docs.wasmer.io">Docs</a>
    <span> • </span>
    <a href="https://slack.wasmer.io/">Slack Channel</a>
  </h3>
</div>

<hr/>

A complete and mature WebAssembly runtime for Ruby based on
[Wasmer](https://github.com/wasmerio/wasmer).

Features

  * **Easy to use**: The `wasmer` API mimics the standard WebAssembly API,
  * **Fast**: `wasmer` executes the WebAssembly modules as fast as
    possible, close to **native speed**,
  * **Safe**: All calls to WebAssembly will be fast, but more
    importantly, completely safe and sandboxed.

**Documentation**: [browse the detailed API
documentation](https://wasmerio.github.io/wasmer-ruby/wasmer_ruby/index.html)
full of examples.

**Examples** as tutorials: [browser the `examples/`
directory](https://github.com/wasmerio/wasmer-ruby/tree/master/examples),
it's the best place for a complete introduction!

# Quick Introduction

The `wasmer` package brings the required API to execute WebAssembly
modules. In a nutshell, `wasmer` compiles the WebAssembly module into
compiled code, and then executes it. `wasmer` is designed to work in
various environments and platforms. To achieve this, Wasmer (the
original runtime) provides multiple engines and multiple
compilers.

Succinctly, an _engine_ is responsible to drive the _compilation_ (by
using a _compiler_) and the _execution_ of a WebAssembly
module. Wasmer comes with many engines and compilers, but for the Ruby
extension, it only provides the JIT engine with the Cranelift
compiler. It is subject to change in the future where the user will
have the choice to select its own engine and compiler.

## Install

To install the `wasmer` Ruby gem, just run this command in your shell:

```sh
$ gem install wasmer
```

And you're ready to get fun!

[View the `wasmer` gem on RubyGems][wasmer-gem].

> Note: [Rust][rust] is required to install the Ruby library (Cargo
—the build tool for Rust— is used to compile the extension). See [how
to install Rust][install-rust].

## Example

We highly recommend to read the
[`examples/`](https://github.com/wasmerio/wasmer-ruby/tree/master/examples)
directory, which contains a sequence of examples/tutorials. It's the
best place to learn by reading examples.

But for the most eager of you, and we know you're numerous you
mischievous, there is a quick toy program in
`examples/appendices/simple.rs`, written in Rust:

```rust
#[no_mangle]
pub extern fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

After compilation to WebAssembly, the
[`examples/appendices/simple.wasm`](https://github.com/wasmerio/wasmer-ruby/blob/master/examples/appendices/simple.wasm)
binary file is generated.

Then, we can execute it in Ruby:

```ruby
require "wasmer"

# Let's define the store, that holds the engine, that holds the compiler.
store = Wasmer::Store.new

# Let's compile the module to be able to execute it!
module_ = Wasmer::Module.new store, IO.read("simple.wasm", mode: "rb")

# Now the module is compiled, we can instantiate it.
instance = Wasmer::Instance.new module_, nil

# Call the exported `sum` function.
result = instance.exports.sum.(5, 37)

puts result # 42!
```

And then, finally, enjoy by running:

```sh
$ ruby examples/appendices/simple.rb
```

## Development

The Ruby extension is written in [Rust][rust], with [`rutie`][rutie].

First, you need to install Rust and Ruby. We will not make you the
affront to explain to you how to install Ruby (if you really need,
check [`rbenv`][rbenv]). For Rust though, we advice to use
[`rustup`][rustup], then:

```sh
$ rustup install stable
```

Then, after installing [`just`][just], you can simply run:

```sh
$ just build
```

The supported Ruby versions are:

* Ruby 2.6,
* Ruby 2.7,
* Ruby 3.0.

### Extensions to `rutie`

This project provides extensions to `rutie`, through the
`rutie-derive` and `rutie-derive-macros` crates (see the `crates/`
directory). Those crates aim at enhancing `rutie` by providing a
better “Domain Specific Language” (DSL for short) to declare Ruby
modules, classes, methods and functions. It's not perfect but it does
a good job for the moment. Contributions on that front are welcomed
too!

### Testing

Running the `test` recipe will automatically build and run all the
tests. It includes library tests, along with documentation tests.

```sh
$ just test
```

### Documentation

To generate the documentation, run the following command:

```sh
$ just doc
```

All the examples in the documentation are automatically run as tests,
called doctests. To run the doctests, run `just test`.

## What is WebAssembly?

Quoting [the WebAssembly site][wasm]:

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

[wasm]: https://webassembly.org/

## License

The entire project is under the MIT License. Please read [the
`LICENSE` file][license].

[license]: https://github.com/wasmerio/wasmer/blob/master/LICENSE

[Wasmer]: https://github.com/wasmerio/wasmer
[rust]: https://www.rust-lang.org/
[install-rust]: https://www.rust-lang.org/tools/install
[wasmer-gem]: https://rubygems.org/gems/wasmer
[rutie]: https://github.com/danielpclark/rutie
[rbenv]: https://github.com/rbenv/rbenv
[rustup]: https://rustup.rs/
[just]: https://github.com/casey/just/
