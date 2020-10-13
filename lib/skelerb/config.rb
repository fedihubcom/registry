# frozen_string_literal: true

module Skelerb
  class Config
    ENVIRONMENT_RE = /\A\w+\z/.freeze

    def root
      @root or raise 'No root specified'
    end

    def environment
      @environment or raise 'No environment specified'
    end

    alias env environment

    def root=(value)
      mutex.synchronize do
        raise 'Attribute already set: root' if @root

        @root = Pathname.new(value).realpath.freeze
      end
    end

    def environment=(value)
      mutex.synchronize do
        raise 'Attribute already set: environment' if @environment

        value = String(value).to_sym
        unless ENVIRONMENT_RE.match? value
          raise "Invalid value: #{value.inspect}"
        end

        @environment = value
      end
    end

    alias env= environment=

  private

    def mutex
      @mutex ||= Mutex.new
    end
  end
end
