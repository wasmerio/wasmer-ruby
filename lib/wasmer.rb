require "fiddle"

module Wasmer
  shared_library_name = :wasmer_ruby
  init_function = :init

  shared_library_prefix =
    case RUBY_PLATFORM
    when /windows|mswin|mingw/ then ""
    when /cygwin/ then "cyg"
    else "lib"
    end

  shared_library_suffix =
    case RUBY_PLATFORM
    when /darwin/ then "dylib"
    when /windows|mswin|mingw|cygwin/ then "dll"
    else "so"
    end

  shared_library_directory = File.expand_path "../target/release/", __dir__
  shared_library_path = File.join(
    shared_library_directory,
    [shared_library_prefix, shared_library_name, ".", shared_library_suffix].join()
  )

  Fiddle::Function.new(
    Fiddle::dlopen(shared_library_path)[init_function.to_s],
    [],
    Fiddle::TYPE_VOIDP
  ).call
end
