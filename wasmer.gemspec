lib = File.expand_path("../lib", __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)

require "wasmer/version"

Gem::Specification.new do |spec|
  spec.name          = "wasmer"
  spec.version       = Wasmer::VERSION
  spec.authors       = ["Ivan Enderlin"]
  spec.email         = ["ivan.enderlin@hoa-project.net"]

  spec.summary       = "Run WebAssembly binaries."
  spec.description   = "Wasmer is a Ruby extension to run WebAssembly binaries."
  spec.homepage      = "https://github.com/wasmerio/ruby-ext-wasm"
  spec.license       = "BSD-3-Clause"

  spec.extensions    = %w(Rakefile)

  spec.files         = Dir.chdir(File.expand_path('..', __FILE__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(examples|tests)/}) }
  end

  spec.require_paths = %w(lib)

  spec.add_dependency "rutie", "~> 0.0.4"
  spec.add_development_dependency "bundler", "~> 2.0"
  spec.add_development_dependency "rake", "~> 10.0"
  spec.add_development_dependency "minitest", "~> 5.0"
end
