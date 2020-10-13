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

    def component(name, &block)
      name = String(name).to_sym

      if block.nil?
        components[name] or raise "No such component: #{name}"
      else
        mutex.synchronize do
          raise "Component already exists: #{name}" if components[name]

          components[name] = Component.new self, name, &block
        end
      end
    end

    alias [] component

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

  private

    def mutex
      @mutex ||= Mutex.new
    end

    def components
      @components ||= {}
    end
  end
end