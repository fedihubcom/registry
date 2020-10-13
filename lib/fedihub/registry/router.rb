# frozen_string_literal: true

module Fedihub
  module Registry
    class Router < Sinatra::Application
      configure :development do
        register Sinatra::Reloader
      end
    end
  end
end
