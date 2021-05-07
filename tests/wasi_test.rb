require "prelude"

class WasiTest < Minitest::Test
  def test_version
    assert_equal Wasi::Version::LATEST, 1
    assert_equal Wasi::Version::SNAPSHOT0, 2
    assert_equal Wasi::Version::SNAPSHOT1, 3
  end

  def test_state_builder
    state_builder = Wasi::StateBuilder.new "test-program"

    state_builder
      .arguments(["--foo", "--bar"])
      .environments({"ABC" => "DEF", "X" => "YZ"})
  end
end
