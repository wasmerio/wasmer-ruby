$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)

require "wasmer"

class AssertionError < RuntimeError
end

def assert &block
  raise AssertionError unless yield
end
