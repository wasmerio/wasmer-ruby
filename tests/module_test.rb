require "prelude"

class ModuleTest < Minitest::Test
  def bytes
    IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def invalid_bytes
    IO.read File.expand_path("invalid.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_validate
    assert Module.validate Store.new, self.bytes
  end

  def test_validate_invalid
    assert not(Module.validate Store.new, self.invalid_bytes)
  end

  def test_compile_bytes
    assert Module.new Store.new, self.bytes
  end

  def test_compile_wat
    assert Module.new Store.new, "(module)"
  end

  def test_failed_to_compile
    assert_raises(RuntimeError) {
      Module.new Store.new, self.invalid_bytes
    }
  end

  def test_name_some
    assert_equal Module.new(Store.new, "(module $moduleName)").name, "moduleName"
  end

  def test_name_none
    assert Module.new(Store.new, "(module)").name.nil?
  end

  def test_name_set
    module_ = Module.new Store.new, "(module)"
    assert module_.name.nil?

    module_.name = "hello"
    assert_equal module_.name, "hello"
  end

  def test_exports
    exports = Module.new(
        Store.new,
        (<<~WAST)
        (module
          (func (export "function") (param i32 i64))
          (global (export "global") i32 (i32.const 7))
          (table (export "table") 0 funcref)
          (memory (export "memory") 1))
        WAST
    ).exports

    assert_equal exports.length, 4
    assert_kind_of ExportType, exports[0]

    assert_equal exports[0].name, "function"
    assert_kind_of FunctionType, exports[0].type
    assert_equal exports[0].type.params, [Type::I32, Type::I64]
    assert_equal exports[0].type.results, [] 

    assert_equal exports[1].name, "global"
    assert_kind_of GlobalType, exports[1].type
    assert_equal exports[1].type.type, Type::I32
    assert_equal exports[1].type.mutable, false

    assert_equal exports[2].name, "table"
    assert_kind_of TableType, exports[2].type
    assert_equal exports[2].type.type, Type::FUNC_REF
    assert_equal exports[2].type.minimum, 0
    assert_nil exports[2].type.maximum

    assert_equal exports[3].name, "memory"
    assert_kind_of MemoryType, exports[3].type
    assert_equal exports[3].type.minimum, 1
    assert_nil exports[3].type.maximum
    assert_equal exports[3].type.shared, false
  end

  def test_imports
    imports = Module.new(
        Store.new,
        (<<~WAST)
        (module
          (import "ns" "function" (func))
          (import "ns" "global" (global f32))
          (import "ns" "table" (table 1 2 anyfunc))
          (import "ns" "memory" (memory 3 4)))
        WAST
    ).imports

    assert_equal imports.length, 4
    assert_kind_of ImportType, imports[0]

    assert_equal imports[0].module, "ns"
    assert_equal imports[0].name, "function"
    assert_kind_of FunctionType, imports[0].type
    assert_equal imports[0].type.params, []
    assert_equal imports[0].type.results, [] 

    assert_equal imports[1].module, "ns"
    assert_equal imports[1].name, "global"
    assert_kind_of GlobalType, imports[1].type
    assert_equal imports[1].type.type, Type::F32
    assert_equal imports[1].type.mutable, false

    assert_equal imports[2].module, "ns"
    assert_equal imports[2].name, "table"
    assert_kind_of TableType, imports[2].type
    assert_equal imports[2].type.type, Type::FUNC_REF
    assert_equal imports[2].type.minimum, 1
    assert_equal imports[2].type.maximum, 2

    assert_equal imports[3].module, "ns"
    assert_equal imports[3].name, "memory"
    assert_kind_of MemoryType, imports[3].type
    assert_equal imports[3].type.minimum, 3
    assert_equal imports[3].type.maximum, 4
    assert_equal imports[3].type.shared, false
  end

  def test_custom_section
    bytes = IO.read File.expand_path("custom_sections.wasm", File.dirname(__FILE__)), mode: "rb"
    module_ = Module.new Store.new, bytes

    assert_equal module_.custom_sections("easter_egg"), ["Wasmer"]
    assert_equal module_.custom_sections("hello"), ["World!"]
    assert_equal module_.custom_sections("foo"), []
  end

  def test_serialize
    module_ = Module.new Store.new, "(module)"
    assert_kind_of String, module_.serialize
  end
end
