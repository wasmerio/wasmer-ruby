require "prelude"

class WasmerTest < Minitest::Test
  def test_it_reverses
    assert_equal 2, Instance.new().call()
  end
end
