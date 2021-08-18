require "bundler/gem_tasks"
require "rake/testtask"

desc 'Build the Rust extension'
task :build_lib do
  sh 'cargo build --release --manifest-path crates/wasmer/Cargo.toml'
end

Rake::TestTask.new(test: :build_lib) do |t|
  t.libs << "tests"
  t.libs << "lib"
  t.test_files = FileList["tests/*_test.rb"]
end

task :default => :build_lib
