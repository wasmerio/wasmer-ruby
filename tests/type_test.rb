require "prelude"

Type = Wasmer::Type
FunctionType = Wasmer::FunctionType

class TypeTest < Minitest::Test
  def test_type
    assert_equal Type::I32, 1
    assert_equal Type::I64, 2
    assert_equal Type::F32, 3
    assert_equal Type::F64, 4
    assert_equal Type::V128, 5
    assert_equal Type::EXTERN_REF, 6
    assert_equal Type::FUNC_REF, 7
  end
end

class FunctionTypeTest < Minitest::Test
  def test_functiontype
    function_type = FunctionType.new [Type::I32, Type::I64], [Type::I32]
    assert_equal function_type.params, [Type::I32, Type::I64]
    assert_equal function_type.results, [Type::I32]
  end
end
