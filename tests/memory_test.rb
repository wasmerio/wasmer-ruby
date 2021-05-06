require "prelude"

class MemoryTest < Minitest::Test
  def instance
    bytes = IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"

    store = Store.new
    module_ = Module.new store, bytes

    Instance.new module_, nil
  end

  def test_export
    assert_kind_of Memory, instance.exports.memory
  end

  def test_type
    type = instance.exports.memory.type

    assert_kind_of MemoryType, type
    assert_equal type.minimum, 17
    assert_nil type.maximum
    assert_equal type.shared?, false
  end

  def test_size
    assert_equal instance.exports.memory.size, 17
  end

  def test_data_size
    assert_equal instance.exports.memory.data_size, 1114112
  end

  def test_grow
    memory = instance.exports.memory

    old_memory_size = memory.data_size

    memory.grow(1)

    memory_size = memory.data_size

    assert_equal memory_size, 1179648
    assert_equal memory_size - old_memory_size, 65536
  end
end
