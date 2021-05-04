require "prelude"

class ModuleTest < Minitest::Test
  TEST_BYTES = 
    (<<~WAST)
    (module
      (global $x (export "x") (mut i32) (i32.const 0))
      (global $y (export "y") (mut i32) (i32.const 7))
      (global $z (export "z") i32 (i32.const 42))
     
      (func (export "get_x") (result i32)
        (global.get $x))
     
      (func (export "increment_x")
        (global.set $x
          (i32.add (global.get $x) (i32.const 1)))))
    WAST

  def test_constructor
    store = Store.new
    global = Global.new store, Value.i32(42), false

    assert global.value, 42

    type = global.type

    assert_equal type.type, Type::I32
    assert_equal type.mutable?, false
  end
end
