require "prelude"

class ModuleTest < Minitest::Test
  def bytes
    IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_new_bytes
    assert Wasmer::Module.new Wasmer::Store.new, self.bytes
  end

  def test_new_wat
    assert Wasmer::Module.new Wasmer::Store.new, "(module)"
  end
end
