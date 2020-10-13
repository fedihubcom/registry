# frozen_string_literal: true

require_relative 'boot'

# Require the gems listed in Gemfile.
Bundler.require

# Require the library files.
Dir[File.expand_path('../lib/**/*.rb', __dir__)].sort.each do |f|
  require f
end

# Require the application files.
Dir[File.expand_path('../app/**/*.rb', __dir__)].sort.each do |f|
  require f
end

# Configure the application.
Skelerb.app.configure do |config|
  config.root = File.expand_path '..', __dir__
  config.environment = ENV['RACK_ENV']
end
