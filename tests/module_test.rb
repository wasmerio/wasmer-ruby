require "prelude"

class ModuleTest < Minitest::Test
  def test_new
    store = Wasmer::Store.new

    assert Wasmer::Module.new store
  end
end
