# `rutie-derive`

**Note**: This is not an official project from
[`rutie`](https://github.com/danielpclark/rutie).

We try to extend `rutie` with a new Domain Specific Language (DSL for
short) based on new procedural macros to make it easier to declare
Ruby modules, classes, methods and functions.

This crate is the front API that provides the `#[ruby*]` procedural
macros from the `rutie-derive-macros` crate, along with some extra
traits and implementations to make everything work magically.

## Ruby class

To declare a new Ruby class, use `#[rubyclass(module = "X::Y::Z")]`:

```rust
#[rubyclass(module = "Wasmer")]
pub struct Foo;
```

It will create a new type `Foo` in Rust, and a new class `Foo` in Ruby
inside the `Wasmer` module.

### Constructor

Every class has a Rust `ruby_new` associated method to create a new
instance of this Ruby type.

## Ruby methods

To declare methods attached to a class, use `#[rubymethods]`:

```rust
#[rubymethods]
impl Foo {
    /// Constructor.
    pub fn new() -> RubyResult<Foo> {
        Ok(Foo::ruby_new(Foo))
    }

    // Method.
    pub fn bar(&self, x: &Integer, y: &Integer) -> RubyResult<Integer> {
        Ok(Integer::new(x.to_i64() + x.to_i64()))
    }
}
```

Ruby methods are like Rust methods:

* Without a receiver (`&self`), it's a static method (`def self`),
* With a receiver (`&self` or `&mut self`), it's a method (`def`),
* All arguments are received by reference,
* The returned type is required to be a `RubyResult`. The error type
  is `AnyException`.

### “Rust-defined” Ruby types

Arguments can be of kind “user-defined types”, let's see:

```rust
#[rubyclass(module = "Wasmer")]
pub struct Baz {
    inner: i32,
};

#[rubyclass(module = "Wasmer")]
pub struct Foo;

#[rubymethods]
impl Foo {
    pub fn bar(&self, baz: &Baz) -> RubyResult<…> {
        let inner = &baz.inner;

        …
    }
}
```

## Ruby functions

Just like `#[rubymethods]`, `#[rubyfunction]` will create a Ruby
function:

```rust
#[rubyfunction]
fn hello(who: &RString) -> RubyResult<RString> {
    Ok(RString::new_utf8(&format!("Hello, {}!", who.to_str())))
}
```

## License

Check the license of
[`wasmer-ruby`](https://github.com/wasmerio/wasmer-ruby/) as it is
part of this same project.
