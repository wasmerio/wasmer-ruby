require "prelude"

class ImportObjectTest < Minitest::Test
  def test_constains_namespace
    import_object = ImportObject.new

    assert_equal import_object.contains_namespace?("foo"), false
  end

  def test_import_function
    def sum(x, y)
      x + y
    end

    store = Store.new
    module_ = Module.new(
      store,
      (<<~WAST)
      (module
        (import "math" "sum" (func $sum (param i32 i32) (result i32)))
        (func (export "add_one") (param i32) (result i32)
          local.get 0
          i32.const 1
          call $sum))
      WAST
    )

    import_object = ImportObject.new
    import_object.register(
      "math",
      {
        :sum => Function.new(store, method(:sum), FunctionType.new([Type::I32, Type::I32], [Type::I32]))
      }
    )

    instance = Instance.new module_, import_object

    assert_equal instance.exports.add_one.(1), 2
  end

  #def test_import_memory
  #end

  def test_import_global
    store = Store.new
    module_ = Module.new(
      store,
      (<<~WAST)
        (module
          (import "env" "global" (global $global (mut i32)))
          (func (export "read_g") (result i32)
            global.get $global)
          (func (export "write_g") (param i32)
            local.get 0
            global.set $global))
      WAST
    )

    global = Global.new store, Value.i32(7), true

    import_object = ImportObject.new
    import_object.register(
      "env",
      {
        :global => global
      }
    )

    instance = Instance.new module_, import_object

    assert_equal instance.exports.read_g.(), 7

    global.value = 153
    assert_equal instance.exports.read_g.(), 153

    instance.exports.write_g.(11)
    assert_equal global.value, 11
  end
end
