require "prelude"

class FunctionTest < Minitest::Test
  def instance
    bytes = IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"

    store = Store.new
    module_ = Module.new store, bytes

    Instance.new module_, nil
  end

  def assert_value_with_type_equal(value, (expected_value, expected_type))
    assert_equal expected_value, value
    assert value.is_a?(expected_type)
  end

  def test_new_with_method_of_symbol
    def foo(x)
      x + 1
    end

    function = Function.new Store.new, method(:foo), FunctionType.new([Type::I32], [])
    assert function
  end

  def test_new_with_proc_as_lambda
    function = Function.new Store.new, -> (x) {}, FunctionType.new([Type::I32], [])
    assert function
  end

  def test_new_with_proc
    function = Function.new Store.new, Proc.new { |x| }, FunctionType.new([Type::I32], [])
    assert function
  end

  def test_export
    assert_kind_of Function, instance.exports.sum
  end

  def test_type
    type = instance.exports.sum.type

    assert_kind_of FunctionType, type
    assert_equal type.params, [Type::I32, Type::I32]
    assert_equal type.results, [Type::I32]
  end

  def test_basic_sum
    assert_value_with_type_equal instance.exports.sum.(1, 2), [3, Integer]
  end

  def test_call_arity_0
    assert_value_with_type_equal instance.exports.arity_0.(), [42, Integer]
  end

  def test_call_i32_i32
    assert_value_with_type_equal instance.exports.i32_i32.(7), [7, Integer]
  end

  def test_call_i64_i64
    assert_value_with_type_equal instance.exports.i64_i64.(7), [7, Integer]
  end

  def test_call_f32_f32
    assert_value_with_type_equal instance.exports.f32_f32.(7.0), [7.0, Float]
  end

  def test_call_f64_f64
    assert_value_with_type_equal instance.exports.i32_i64_f32_f64_f64.(1, 2, 3.4, 5.6).round(6), [1 + 2 + 3.4 + 5.6, Float]
  end

  def test_call_bool_casted_to_i32
    assert_value_with_type_equal instance.exports.bool_casted_to_i32.(), [1, Integer]
  end

  def test_call_string
    assert_equal instance.exports.string.(), 1048576
  end

  def test_call_void
    assert_nil instance.exports.void.()
  end

  #def test_early_exit
  #  store = Store.new
  #  module_ = Module.new(
  #    store,
  #    (<<~WAST)
  #    (module
  #      (type $run_t (func (param i32 i32) (result i32)))
  #      (type $early_exit_t (func (param) (result)))

  #      (import "env" "early_exit" (func $early_exit (type $early_exit_t)))

  #      (func $run (type $run_t) (param $x i32) (param $y i32) (result i32)
  #        (call $early_exit)
  #        (i32.add
  #            local.get $x
  #            local.get $y))

  #      (export "run" (func $run)))
  #    WAST
  #  )

  #  def eary_exit(x)
  #    raise "oops"
  #  end
  #end
end
