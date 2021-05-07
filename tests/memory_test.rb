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

  def test_typed_arrays
    memory = instance.exports.memory

    assert_kind_of Int8Array, memory.int8_view(0)
    assert_kind_of Uint8Array, memory.uint8_view(0)
    assert_kind_of Int16Array, memory.int16_view(0)
    assert_kind_of Uint16Array, memory.uint16_view(0)
    assert_kind_of Int32Array, memory.int32_view(0)
    assert_kind_of Uint32Array, memory.uint32_view(0)
  end

  def test_typed_arrays_bytes_per_element
    assert_equal 1, Int8Array::BYTES_PER_ELEMENT
    assert_equal 1, Uint8Array::BYTES_PER_ELEMENT
    assert_equal 2, Int16Array::BYTES_PER_ELEMENT
    assert_equal 2, Uint16Array::BYTES_PER_ELEMENT
    assert_equal 4, Int32Array::BYTES_PER_ELEMENT
    assert_equal 4, Uint32Array::BYTES_PER_ELEMENT
  end

  def test_typed_array_length
    assert_equal instance.exports.memory.uint8_view(0).length, 1114112
  end

  def test_typed_array_get_set_index
    memory = instance.exports.memory.uint8_view(0)
    index = 7
    value = 42
    memory[index] = value

    assert_equal memory[index], value
  end

  def test_typed_array_get_out_of_bound
    memory = instance.exports.memory.uint8_view(0)

    assert_raises(IndexError) {
      memory[-1]
    }

    assert_raises(IndexError) {
      memory[memory.length]
    }
  end

  def test_typed_array_set_out_of_bound
    memory = instance.exports.memory.uint8_view(0)

    assert_raises(IndexError) {
      memory[-1] = 0
    }

    assert_raises(IndexError) {
      memory[memory.length] = 0
    }
  end

  def test_hello_world
    i = instance
    pointer = i.exports.string.()
    memory = i.exports.memory.uint8_view pointer
    nth = 0
    string = ""

    memory.each do |char|
      break if 0 == char
      string << char.chr
      nth += 1
    end

    assert_equal nth, 13
    assert_equal string, "Hello, World!"
  end

  def test_typed_array_enumerable
    memory = instance.exports.memory.int16_view(0)
    memory[0] = 1
    memory[1] = 10
    memory[2] = 100
    memory[3] = 1000
    memory[5] = 2
    sum = memory.take_while{|x| x > 0}.inject(0, &:+)

    assert_equal sum, 1111
  end

  def test_typed_arrays_share_the_same_buffer
    memory = instance.exports.memory
    int8 = memory.int8_view(0)
    int16 = memory.int16_view(0)
    int32 = memory.int32_view(0)

    int8[0] = 0b00000001
    int8[1] = 0b00000100
    int8[2] = 0b00010000
    int8[3] = 0b01000000

    assert_equal int8[0], 0b00000001
    assert_equal int8[1], 0b00000100
    assert_equal int8[2], 0b00010000
    assert_equal int8[3], 0b01000000
    assert_equal int16[0], 0b00000100_00000001
    assert_equal int16[1], 0b01000000_00010000
    assert_equal int32[0], 0b01000000_00010000_00000100_00000001
  end
end
