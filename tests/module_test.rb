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
end
