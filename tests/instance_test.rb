require "prelude"

class InstanceTest < Minitest::Test
  def bytes
    IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_can_construct
    assert Instance.new self.bytes
  end

  def test_basic_sum
    assert_equal 3, Instance.new(self.bytes).exports.sum(1, 2)
  end

  def test_call_unknown_function
    error = assert_raises(RuntimeError) {
      Instance.new(self.bytes).exports.foo
    }
    assert_equal "Function `foo` does not exist.", error.message
  end

  def test_call_missing_argument
    error = assert_raises(ArgumentError) {
      Instance.new(self.bytes).exports.sum 1
    }
    assert_equal "Missing 1 argument(s) when calling `sum`: Expect 2 argument(s), given 1.", error.message
  end

  def test_call_extra_argument
    error = assert_raises(ArgumentError) {
      Instance.new(self.bytes).exports.sum 1, 2, 3
    }
    assert_equal "Given 1 extra argument(s) when calling `sum`: Expect 2 argument(s), given 3.", error.message
  end

  def test_call_arity_0
    assert_equal 42, Instance.new(self.bytes).exports.arity_0
  end

  def test_call_i32_i32
    assert_equal 7, Instance.new(self.bytes).exports.i32_i32(7)
  end

  def test_call_i64_i64
    assert_equal 7, Instance.new(self.bytes).exports.i64_i64(7)
  end

  def test_call_f32_f32
    assert_equal 7.0, Instance.new(self.bytes).exports.f32_f32(7.0)
  end

  def test_call_f64_f64
    assert_equal 7.0, Instance.new(self.bytes).exports.f64_f64(7.0)
  end

  def test_call_i32_i64_f32_f64_f64
    assert_equal 1 + 2 + 3.4 + 5.6, Instance.new(self.bytes).exports.i32_i64_f32_f64_f64(1, 2, 3.4, 5.6).round(6)
  end

  def test_call_bool_casted_to_i32
    assert_equal 1, Instance.new(self.bytes).exports.bool_casted_to_i32
  end

  def test_call_string
    assert_equal 1048576, Instance.new(self.bytes).exports.string
  end

  def test_exports
    assert_instance_of ExportedFunctions, Instance.new(self.bytes).exports
  end

  def test_memory
    assert_instance_of Memory, Instance.new(self.bytes).memory
  end
end
