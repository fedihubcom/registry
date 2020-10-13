# frozen_string_literal: true

require_relative 'config/environment'

require 'rom/sql/rake_task'

namespace :db do
  task :setup do
    ROM::SQL::RakeSupport.env = ROM.container Skelerb.app[:database].object
  end
end
