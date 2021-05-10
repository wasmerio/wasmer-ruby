require "prelude"

class WasiTest < Minitest::Test
  def test_version
    assert_equal Wasi::Version::LATEST, 1
    assert_equal Wasi::Version::SNAPSHOT0, 2
    assert_equal Wasi::Version::SNAPSHOT1, 3
  end

  def test_state_builder
    state_builder = Wasi::StateBuilder.new("test-program")
      .arguments(["--foo", "--bar"])
      .environments({"ABC" => "DEF", "X" => "YZ"})
      .map_directory("the_host_current_dir", ".")

    assert_kind_of Wasi::StateBuilder, state_builder
  end
end
