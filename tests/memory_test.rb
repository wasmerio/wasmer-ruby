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
end
