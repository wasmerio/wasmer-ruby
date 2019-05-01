require "prelude"

class MemoryTest < Minitest::Test
  def bytes
    IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_memory
    assert_instance_of Memory, Instance.new(self.bytes).memory
  end

  def test_typedarrays
    assert_instance_of Uint8Array, Instance.new(self.bytes).memory.uint8_view
    assert_instance_of Int8Array, Instance.new(self.bytes).memory.int8_view
    assert_instance_of Uint16Array, Instance.new(self.bytes).memory.uint16_view
    assert_instance_of Int16Array, Instance.new(self.bytes).memory.int16_view
    assert_instance_of Uint32Array, Instance.new(self.bytes).memory.uint32_view
    assert_instance_of Int32Array, Instance.new(self.bytes).memory.int32_view
  end

  def test_bytes_per_element
    assert_equal 1, Instance.new(self.bytes).memory.uint8_view.bytes_per_element
    assert_equal 1, Instance.new(self.bytes).memory.int8_view.bytes_per_element
    assert_equal 2, Instance.new(self.bytes).memory.uint16_view.bytes_per_element
    assert_equal 2, Instance.new(self.bytes).memory.int16_view.bytes_per_element
    assert_equal 4, Instance.new(self.bytes).memory.uint32_view.bytes_per_element
    assert_equal 4, Instance.new(self.bytes).memory.int32_view.bytes_per_element
  end

  def test_view_with_offset
    assert_instance_of Uint8Array, Instance.new(self.bytes).memory.uint8_view(7)
  end

  def test_length
    assert_equal 1114112, Instance.new(self.bytes).memory.uint8_view.length
  end

  def test_get
    memory = Instance.new(self.bytes).memory.uint8_view
    index = 0
    value = 42
    memory[index] = value

    assert_equal value, memory[index]
  end

  def test_get_invalid_index_type
    error = assert_raises(TypeError) {
      Instance.new(self.bytes).memory.uint8_view["-1"]
    }
    assert_equal "Error converting to Integer", error.message
  end

  def test_get_out_of_bound_negative
    error = assert_raises(ArgumentError) {
      Instance.new(self.bytes).memory.uint8_view[-1]
    }
    assert_equal "Out of bound: Index cannot be negative.", error.message
  end

  def test_get_out_of_bound_too_large
    error = assert_raises(ArgumentError) {
      memory = Instance.new(self.bytes).memory.uint8_view
      length = memory.length

      Instance.new(self.bytes).memory.uint8_view[length + 1]
    }
    assert_equal "Out of bound: Maximum index 1114113 is larger than the memory size 1114112.", error.message
  end

  def test_set_invalid_index_type
    error = assert_raises(TypeError) {
      Instance.new(self.bytes).memory.uint8_view["-1"] = 1
    }
    assert_equal "Error converting to Integer", error.message
  end

  def test_set_out_of_bound_negative
    error = assert_raises(ArgumentError) {
      Instance.new(self.bytes).memory.uint8_view[-1] = 1
    }
    assert_equal "Out of bound: Index cannot be negative.", error.message
  end

  def test_set_out_of_bound_too_large
    error = assert_raises(ArgumentError) {
      memory = Instance.new(self.bytes).memory.uint8_view
      length = memory.length

      Instance.new(self.bytes).memory.uint8_view[length + 1] = 1
    }
    assert_equal "Out of bound: Maximum index 1114113 is larger than the memory size 1114112.", error.message
  end

  def test_hello_world
    instance = Instance.new(self.bytes)
    pointer = instance.exports.string
    memory = instance.memory.uint8_view pointer
    nth = 0
    string = ""

    while true
      char = memory[nth]

      if 0 == char
        break
      end

      string += char.chr
      nth += 1
    end

    assert_equal "Hello, World!", string
  end

  def test_views_share_the_same_buffer
    instance = Instance.new self.bytes
    int8 = instance.memory.int8_view
    int16 = instance.memory.int16_view
    int32 = instance.memory.int32_view

    int8[0] = 0b00000001
    int8[1] = 0b00000100
    int8[2] = 0b00010000
    int8[3] = 0b01000000

    assert_equal 0b00000001, int8[0]
    assert_equal 0b00000100, int8[1]
    assert_equal 0b00010000, int8[2]
    assert_equal 0b01000000, int8[3]
    assert_equal 0b00000100_00000001, int16[0]
    assert_equal 0b01000000_00010000, int16[1]
    assert_equal 0b01000000_00010000_00000100_00000001, int32[0]
  end
end
