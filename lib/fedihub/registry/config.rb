# frozen_string_literal: true

module Fedihub
  module Registry
    class Config
      ENVIRONMENT_RE = /\A\w+\z/.freeze

      def root
        @root or raise 'No root specified'
      end

      def environment
        @environment or raise 'No environment specified'
      end

      def root=(value)
        @root = Pathname.new(value).realpath.freeze
      end

      def environment=(value)
        value = String(value).to_sym
        value = default_environment if value.empty?
        unless ENVIRONMENT_RE.match? value
          raise "Invalid value: #{value.inspect}"
        end

        @environment = value
      end
    end
  end
end
