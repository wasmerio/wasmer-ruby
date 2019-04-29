require "prelude"

class ModuleTest < Minitest::Test
  def bytes
    IO.read File.expand_path("tests.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_view
    assert_instance_of MemoryView, Instance.new(self.bytes).memory.view
  end

  def test_view_with_offset
    assert_instance_of MemoryView, Instance.new(self.bytes).memory.view(7)
  end

  def test_length
    assert_equal 1114112, Instance.new(self.bytes).memory.view.length
  end

  def test_get
    memory = Instance.new(self.bytes).memory.view
    index = 0
    value = 42
    memory.set(index, value)

    assert_equal value, memory.get(index)
  end

  def test_hello_world
    instance = Instance.new(self.bytes)
    pointer = instance.exports.string
    memory = instance.memory.view(pointer)
    nth = 0
    string = ''

    while 0 != memory.get(nth)
      string += memory.get(nth).chr
      nth += 1
    end

    assert_equal "Hello, World!", string
  end
end
