require "prelude"

class GlobalTest < Minitest::Test
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

  def instance
    Instance.new Module.new(Store.new, TEST_BYTES), nil
  end
  
  def test_constructor
    store = Store.new
    global = Global.new store, Value.i32(42), false

    assert_equal global.value, 42

    type = global.type

    assert_equal type.type, Type::I32
    assert_equal type.mutable?, false
  end

  def test_constructor_mutable
    store = Store.new
    global = Global.new store, Value.i32(42), true

    assert_equal global.value, 42

    type = global.type

    assert_equal type.type, Type::I32
    assert_equal type.mutable?, true

    global.value = 153

    assert_equal global.value, 153
  end

  def test_export
    assert_kind_of Global, instance.exports.x
  end

  def test_type
    type = instance.exports.x.type

    assert_equal type.type, Type::I32
    assert_equal type.mutable?, true
  end

  def test_global_read_write
    y = instance.exports.y

    assert_equal y.value, 7

    y.value = 8

    assert_equal y.value, 8
  end

  #def test_global_read_write_and_exported_functions
  #end

  def test_global_read_write_constant
    z = instance.exports.z

    assert_equal z.value, 42

    assert_raises(RuntimeError) {
      z.value = 153
    }
  end
end
