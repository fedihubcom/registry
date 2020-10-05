# frozen_string_literal: true

# Load the application.
require_relative 'application'

# Initialize the application.
Dir[File.join(__dir__, 'initializers', '*.rb')].each { |f| require f }
