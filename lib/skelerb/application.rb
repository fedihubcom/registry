# frozen_string_literal: true

module Skelerb
  class Application
    def initialize!
      Dir[config.root.join('config', 'initializers', '*.rb')].sort.each do |f|
        require f
      end
    end

    def config
      @config ||= Config.new
    end

    def configure
      yield config
    end

    def rack
      @rack ||= Rack::Builder.new.tap do |rack|
        rack.run router
      end
    end

    def router
      config = self.config

      @router ||= Class.new Sinatra::Application do
        set :environment, config.environment
      end
    end
  end
end
