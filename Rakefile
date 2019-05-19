require "rbconfig"
require "bundler/gem_tasks"
require "rake/testtask"
require "thermite/tasks"

Rake::TestTask.new(test: ["thermite:build", "thermite:test"]) do |t|
  t.libs << "tests"
  t.libs << "lib"
  t.test_files = FileList["tests/**/*_test.rb"]
end

Thermite::Tasks.new

task :default => :test
