require "prelude"

Type = Wasmer::Type
FunctionType = Wasmer::FunctionType
MemoryType = Wasmer::MemoryType
GlobalType = Wasmer::GlobalType
TableType = Wasmer::TableType
ExportType = Wasmer::ExportType

class TypeTest < Minitest::Test
  def test_type
    assert_equal Type::I32 , 1
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

class MemoryTypeTest < Minitest::Test
  def test_memorytype
    memory_type = MemoryType.new 1, 2, true
    assert_equal memory_type.minimum, 1
    assert_equal memory_type.maximum, 2
    assert_equal memory_type.shared, true
  end

  def test_unbound_memorytype
    memory_type = MemoryType.new 1, nil, false
    assert_equal memory_type.minimum, 1
    assert_nil memory_type.maximum
    assert_equal memory_type.shared, false
  end
end

class GlobalTypeTest < Minitest::Test
  def test_globaltype
    global_type = GlobalType.new Type::I32, true
    assert_equal global_type.type, Type::I32
    assert_equal global_type.mutable, true
  end
end

class TableTypeTest < Minitest::Test
  def test_tabletype
    table_type = TableType.new Type::I32, 1, 2
    assert_equal table_type.type, Type::I32
    assert_equal table_type.minimum, 1
    assert_equal table_type.maximum, 2
  end

  def test_unbound_tabletype
    table_type = TableType.new Type::I32, 1, nil
    assert_equal table_type.type, Type::I32
    assert_equal table_type.minimum, 1
    assert_nil table_type.maximum
  end
end

class ExportTypeTest < Minitest::Test
  def test_exporttype
    export_type = ExportType.new "foo", (FunctionType.new [Type::I32], [])
    assert_equal export_type.name, "foo"

    function_type = export_type.type
    assert_kind_of FunctionType, function_type
    assert_equal function_type.params, [Type::I32]
    assert_equal function_type.results, []
  end
end
