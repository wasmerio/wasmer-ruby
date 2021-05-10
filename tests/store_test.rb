require "prelude"

class StoreTest < Minitest::Test
  def test_new
    assert Store.new
  end
end
