# frozen_string_literal: true

module Fedihub
  module Registry
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
        @rack ||= Rack::Builder.new do
          use Rack::Attack

          run Router
        end
      end

      def router
        @router ||= Class.new Router do
          set :environment, config.environment
        end
      end
    end
  end
end
