# coding: utf-8
require "prelude"

class GlobalTest < Minitest::Test
  def bytes
    IO.read File.expand_path("global.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_global_mutable
    globals = Wasmer::Instance.new(self.bytes).globals

    assert globals.x.mutable == true
    assert globals.y.mutable == true
    assert globals.z.mutable == false
  end

  def test_global_read_write
    y = Wasmer::Instance.new(self.bytes).globals.y

    assert y.value == 7

    y.value = 8

    assert y.value == 8
  end

  def test_global_write_invalid_type
    y = Wasmer::Instance.new(self.bytes).globals.y

    error = assert_raises(TypeError) {
      y.value = 4.2
    }
    assert_equal "Failed to set `AnyObject { value: Value { value: 37830236869912170 } }` to the global `y` (with type `I32`).", error.message
  end

  def test_global_read_write_and_exported_functions
    instance = Wasmer::Instance.new self.bytes
    exports = instance.exports
    x = instance.globals.x

    assert x.value == 0
    assert exports.get_x == 0

    x.value = 1

    assert x.value == 1
    assert exports.get_x == 1

    exports.increment_x

    assert x.value == 2
    assert exports.get_x == 2
  end

  def test_global_read_write_constants
    z = Wasmer::Instance.new(self.bytes).globals.z

    assert z.value == 42

    error = assert_raises(RuntimeError) {
      z.value = 153
    }
    assert_equal "The global variable `z` is not mutable, cannot set a new value.", error.message
  end
end
