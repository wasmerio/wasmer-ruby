require "prelude"

class InstanceTest < Minitest::Test
  def bytes
    IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def invalid_bytes
    IO.read File.expand_path("invalid.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_can_construct
    assert Wasmer::Instance.new self.bytes
  end

  def test_constructor_needs_bytes
    error = assert_raises(ArgumentError) {
      Wasmer::Instance.new 123
    }
    assert_equal "WebAssembly module must be represented by Ruby bytes only.", error.message
  end

  def test_module_must_have_an_exported_memory
    error = assert_raises(RuntimeError) {
      bytes = IO.read File.expand_path("no_memory.wasm", File.dirname(__FILE__)), mode: "rb"
      Wasmer::Instance.new bytes
    }
    assert_equal "The WebAssembly module has no exported memory.", error.message
  end

  def test_invalid_module
    error = assert_raises(RuntimeError) {
      Wasmer::Instance.new self.invalid_bytes
    }
    assert_equal "Failed to instantiate the module:\n    compile error: Validation error \"Invalid type\"", error.message
  end

  def test_basic_sum
    exports = Wasmer::Instance.new(self.bytes).exports
    assert exports.respond_to?(:sum)
    assert_equal 3, exports.sum(1, 2)
  end

  def test_call_unknown_function
    exports = Wasmer::Instance.new(self.bytes).exports
    assert !exports.respond_to?(:foo)
    error = assert_raises(RuntimeError) {
      exports.foo
    }
    assert_equal "Function `foo` does not exist.", error.message
  end

  def test_call_missing_argument
    error = assert_raises(ArgumentError) {
      Wasmer::Instance.new(self.bytes).exports.sum 1
    }
    assert_equal "Missing 1 argument(s) when calling `sum`: Expect 2 argument(s), given 1.", error.message
  end

  def test_call_extra_argument
    error = assert_raises(ArgumentError) {
      Wasmer::Instance.new(self.bytes).exports.sum 1, 2, 3
    }
    assert_equal "Given 1 extra argument(s) when calling `sum`: Expect 2 argument(s), given 3.", error.message
  end

  def test_call_cannot_convert_argument
    error = assert_raises(ArgumentError) {
      Wasmer::Instance.new(self.bytes).exports.sum 1, "2"
    }
    assert_equal "Cannot convert argument #2 to a WebAssembly value. Only integers and floats are supported. Given `RString`.", error.message
  end

  def test_call_arity_0
    assert_equal 42, Wasmer::Instance.new(self.bytes).exports.arity_0
  end

  def test_call_i32_i32
    assert_equal 7, Wasmer::Instance.new(self.bytes).exports.i32_i32(7)
  end

  def test_call_i64_i64
    assert_equal 7, Wasmer::Instance.new(self.bytes).exports.i64_i64(7)
  end

  def test_call_f32_f32
    assert_equal 7.0, Wasmer::Instance.new(self.bytes).exports.f32_f32(7.0)
  end

  def test_call_f64_f64
    assert_equal 7.0, Wasmer::Instance.new(self.bytes).exports.f64_f64(7.0)
  end

  def test_call_i32_i64_f32_f64_f64
    assert_equal 1 + 2 + 3.4 + 5.6, Wasmer::Instance.new(self.bytes).exports.i32_i64_f32_f64_f64(1, 2, 3.4, 5.6).round(6)
  end

  def test_call_bool_casted_to_i32
    assert_equal 1, Wasmer::Instance.new(self.bytes).exports.bool_casted_to_i32
  end

  def test_call_string
    assert_equal 1048576, Wasmer::Instance.new(self.bytes).exports.string
  end

  def test_call_void
    assert_equal nil, Wasmer::Instance.new(self.bytes).exports.void
  end

  def test_exports
    assert_instance_of Wasmer::ExportedFunctions, Wasmer::Instance.new(self.bytes).exports
  end

  def test_memory
    assert_instance_of Wasmer::Memory, Wasmer::Instance.new(self.bytes).memory
  end
end
