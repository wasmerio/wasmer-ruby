require "prelude"

class ValueTest < Minitest::Test
  def test_i32
    assert Value.i32 42
  end

  def test_i64
    assert Value.i64 42
  end

  def test_f32
    assert Value.f32 4.2
  end

  def test_f64
    assert Value.f64 4.2
  end
end
