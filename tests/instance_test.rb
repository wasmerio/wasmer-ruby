require "prelude"

class InstanceTest < Minitest::Test
  def test_new
    assert Instance.new Module.new(Store.new, "(module)"), nil
  end

  def test_exports
    instance = Instance.new Module.new(Store.new, "(module)"), nil

    assert_kind_of Exports, instance.exports
  end

  def test_exports_all_kind
    module_ = Module.new(
      Store.new,
      (<<~WAST)
      (module
        (func (export "func") (param i32 i64))
        (global (export "glob") i32 (i32.const 7))
        (table (export "tab") 0 funcref)
        (memory (export "mem") 1))
      WAST
    )
    instance = Instance.new module_, nil
    exports = instance.exports

    assert exports.respond_to? :func
    assert exports.respond_to? :glob
    assert exports.respond_to? :tab
    assert exports.respond_to? :mem
    assert not(exports.respond_to? :foo)

    assert_kind_of Function, exports.func
    assert_kind_of Memory, exports.mem
  end
end
