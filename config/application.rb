# frozen_string_literal: true

require_relative 'boot'

# Require the gems listed in Gemfile.
Bundler.require

module Fedihub
  module Registry
    class Application < Sinatra::Application
      configure :development do
        register Sinatra::Reloader
      end
    end
  end
end
