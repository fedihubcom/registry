# frozen_string_literal: true

module Fedihub
  module Registry
    class Application
      ENVIRONMENT_RE = /\A\w+\z/.freeze

      def root
        @root or raise 'No root specified'
      end

      def default_environment
        @default_environment or raise 'No default environment specified'
      end

      def environment
        @environment or raise 'No environment specified'
      end

      def root=(value)
        @root = Pathname.new(value).realpath.freeze
      end

      def default_environment=(value)
        value = String(value).to_sym
        unless ENVIRONMENT_RE.match? value
          raise "Invalid value: #{value.inspect}"
        end

        @default_environment = value
      end

      def environment=(value)
        value = String(value).to_sym
        value = default_environment if value.empty?
        unless ENVIRONMENT_RE.match? value
          raise "Invalid value: #{value.inspect}"
        end

        @environment = value
      end

      def initialize!
        Dir[root.join('config', 'initializers', '*.rb')].sort.each do |f|
          require f
        end
      end

      def rack
        @rack ||= Rack::Builder.new do
          use Rack::Attack

          run Router
        end
      end

      def router
        @router ||= Class.new Router do
          set :environment, environment
        end
      end
    end
  end
end
