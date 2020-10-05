# frozen_string_literal: true

require_relative 'boot'

# Require the gems listed in Gemfile.
Bundler.require

module Fedihub
  module Webapp
    class Application < Sinatra::Application
    end
  end
end
