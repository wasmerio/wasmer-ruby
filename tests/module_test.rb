require "prelude"

class ModuleTest < Minitest::Test
  def bytes
    IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def invalid_bytes
    IO.read File.expand_path("invalid.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_validate
    assert Wasmer::Module.validate Wasmer::Store.new, self.bytes
  end

  def test_validate_invalid
    assert not(Wasmer::Module.validate Wasmer::Store.new, self.invalid_bytes)
  end

  def test_compile_bytes
    assert Wasmer::Module.new Wasmer::Store.new, self.bytes
  end

  def test_compile_wat
    assert Wasmer::Module.new Wasmer::Store.new, "(module)"
  end

  def test_failed_to_compile
    assert_raises(RuntimeError) {
      Wasmer::Module.new Wasmer::Store.new, self.invalid_bytes
    }
  end

  def test_name_some
    assert_equal Wasmer::Module.new(Wasmer::Store.new, "(module $moduleName)").name, "moduleName"
  end

  def test_name_none
    assert Wasmer::Module.new(Wasmer::Store.new, "(module)").name.nil?
  end

  def test_name_set
    module_ = Wasmer::Module.new Wasmer::Store.new, "(module)"
    assert module_.name.nil?

    module_.name = "hello"
    assert_equal module_.name, "hello"
  end

  def test_custom_section
    bytes = IO.read File.expand_path("custom_sections.wasm", File.dirname(__FILE__)), mode: "rb"
    module_ = Wasmer::Module.new Wasmer::Store.new, bytes

    assert_equal module_.custom_sections("easter_egg"), ["Wasmer"]
    assert_equal module_.custom_sections("hello"), ["World!"]
    assert_equal module_.custom_sections("foo"), []
  end
end
