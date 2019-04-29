$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)

require "wasmer"

file = File.expand_path "memory.wasm", File.dirname(__FILE__)
bytes = IO.read file, mode: "rb"
instance = Instance.new bytes
pointer = instance.exports.return_hello

memory = instance.memory.uint8_view pointer
nth = 0
string = ""

while true
  char = memory[nth]

  if 0 == char
    break
  end
  
  string += char.chr
  nth += 1
end

puts string
