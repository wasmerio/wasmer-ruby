require "rbconfig"
require "bundler/gem_tasks"
require "rake/testtask"

desc 'Build the Rust extension'
task :build_lib do
  sh 'cargo build --release'
end

desc 'Install the bundle'
task :bundle_install do
  sh 'bundle install'
end

Rake::TestTask.new(test: [:bundle_install, :build_lib]) do |t|
  t.libs << "tests"
  t.libs << "lib"
  t.test_files = FileList["tests/**/*_test.rb"]
end

task :default => :test
