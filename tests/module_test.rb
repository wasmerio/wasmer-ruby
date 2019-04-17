require "prelude"

class ModuleTest < Minitest::Test
  def bytes
    IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def invalid_bytes
    IO.read File.expand_path("invalid.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_validate
    assert Module.validate self.bytes
  end

  def test_validate_invalid_bytes
    assert_equal false, Module.validate(self.invalid_bytes)
  end

  def test_validate_invalid_type
    assert_equal false, Module.validate(42)
  end
end
