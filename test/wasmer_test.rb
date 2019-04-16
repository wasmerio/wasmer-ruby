require "prelude"

class WasmerTest < Minitest::Test
  def test_it_reverses
    assert_equal "hello", Instance.echo("hello")
  end
end
