require "prelude"

class WatTest < Minitest::Test
  def test_wat2wasm
    assert_equal Wasmer::wat2wasm("(module)"), "\x00asm\x01\x00\x00\x00"
  end

  def test_wasm2wat
    assert_equal Wasmer::wasm2wat("\x00asm\x01\x00\x00\x00"), "(module)"
  end

  def test_wat2wasm2instance
    wat =
      (<<~WAST)
      (module
        (type (func (param i32 i32) (result i32)))
        (func (type 0)
          local.get 0
          local.get 1
          i32.add)
        (export "sum" (func 0)))
      WAST
    wasm_bytes = Wasmer::wat2wasm wat
    instance = Instance.new Module.new(Store.new, wasm_bytes), nil

    assert_equal instance.exports.sum.(1, 2), 3
  end
end
