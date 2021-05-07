require "prelude"

class WasiTest < Minitest::Test
  def test_version
    assert_equal Wasi::Version::LATEST, 1
    assert_equal Wasi::Version::SNAPSHOT0, 2
    assert_equal Wasi::Version::SNAPSHOT1, 3
  end
end
