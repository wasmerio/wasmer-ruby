$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)

require "wasmer"

file = File.expand_path "simple.wasm", File.dirname(__FILE__)
bytes = IO.read file, mode: "rb"
instance = Wasmer::Instance.new bytes
puts instance.exports.sum 1, 2
