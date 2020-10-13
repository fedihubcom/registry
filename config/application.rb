# frozen_string_literal: true

require_relative 'boot'

# Require the gems listed in Gemfile.
Bundler.require

# Require the application files.
Dir[File.expand_path('../lib/**/*.rb', __dir__)].sort.each do |f|
  require f
end

# Configure the application.
Fedihub::Registry.application.root = File.expand_path '..', __dir__
Fedihub::Registry.application.default_environment = :development
Fedihub::Registry.application.environment = ENV['RACK_ENV']
