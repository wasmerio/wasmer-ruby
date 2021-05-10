require "prelude"

class WasiTest < Minitest::Test
  def bytes
    IO.read File.expand_path("wasi.wasm", File.dirname(__FILE__)), mode: "rb"
  end

  def test_version
    assert_equal Wasi::Version::LATEST, 1
    assert_equal Wasi::Version::SNAPSHOT0, 2
    assert_equal Wasi::Version::SNAPSHOT1, 3
  end

  def test_get_version
    module_ = Module.new(Store.new, bytes)

    assert_equal Wasi::get_version(module_, true), Wasi::Version::SNAPSHOT1
  end

  def test_state_builder
    state_builder = Wasi::StateBuilder.new("test-program")
      .arguments(["--foo", "--bar"])
      .environments({"ABC" => "DEF", "X" => "YZ"})
      .map_directory("the_host_current_dir", ".")

    assert_kind_of Wasi::StateBuilder, state_builder
  end

  def test_environment
    assert_kind_of Wasi::Environment, Wasi::StateBuilder.new("foo").finalize
  end
end
