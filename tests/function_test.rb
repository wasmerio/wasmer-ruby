require "prelude"

class FunctionTest < Minitest::Test
  def instance
    bytes = IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"

    store = Store.new
    module_ = Module.new store, bytes

    Instance.new module_, nil
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
end
