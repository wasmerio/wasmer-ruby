require "prelude"

class ModuleTest < Minitest::Test
  def valid_bytes
    IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def invalid_bytes
    IO.read File.expand_path("invalid.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_validate
    assert Wasmer::Module.validate self.valid_bytes
  end

  def test_validate_invalid_bytes
    assert_equal false, Wasmer::Module.validate(self.invalid_bytes)
  end

  def test_validate_invalid_type
    assert_equal false, Wasmer::Module.validate(42)
  end
end
