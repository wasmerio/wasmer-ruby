# coding: utf-8
$LOAD_PATH.unshift File.expand_path("../../../lib", __FILE__)

require "wasmer"

# Instantiates the module.
file = File.expand_path "greet.wasm", File.dirname(__FILE__)
bytes = IO.read file, mode: "rb"

store = Wasmer::Store.new
module_ = Wasmer::Module.new store, bytes
instance = Wasmer::Instance.new module_, nil

# Set the subject to greet.
subject = "Wasmer 💎".bytes
length_of_subject = subject.length

# Allocate memory for the subject, and get a pointer to it.
input_pointer = instance.exports.allocate.(length_of_subject)

# Write the subject into the memory.
memory = instance.exports.memory.uint8_view input_pointer

for nth in 0..length_of_subject - 1
  memory[nth] = subject[nth]
end

# C-string terminates by NULL.
memory[length_of_subject] = 0

# Run the `greet` function. Give the pointer to the subject.
output_pointer = instance.exports.greet.(input_pointer)

# Read the result of the `greet` function.
memory = instance.exports.memory.uint8_view output_pointer

output = ""
nth = 0

while true
  char = memory[nth]

  if 0 == char
    break
  end

  output += char.chr
  nth += 1
end

length_of_output = nth

puts output

# Deallocate the subject, and the output.
instance.exports.deallocate.(input_pointer, length_of_subject)
instance.exports.deallocate.(output_pointer, length_of_output)
