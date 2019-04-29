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

  def test_view_length
    assert_equal 1114112, Instance.new(self.bytes).memory.view.length
  end
end
