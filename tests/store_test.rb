require "prelude"

class StoreTest < Minitest::Test
  def test_new
    assert Wasmer::Store.new
  end
end
