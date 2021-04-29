require "prelude"

Module = Wasmer::Module
Store = Wasmer::Store

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
