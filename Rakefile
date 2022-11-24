# frozen_string_literal: true

require 'bundler/gem_tasks'
require 'rake/testtask'
require 'rake/extensiontask'

Rake::ExtensionTask.new('rumo') do |c|
  c.lib_dir = 'lib/rumo'
end

Rake::TestTask.new(:test) do |t|
  t.libs << 'test'
  t.libs << 'lib'
  t.test_files = FileList['test/**/*_test.rb']
end

desc 'Expand erb templates'
task :expand do
  `bundle exec ruby ext/rumo/src/lib.erb.rs`
  `bundle exec ruby test/rumo_test.erb`
end

task default: :test
