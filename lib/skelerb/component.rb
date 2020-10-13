# frozen_string_literal: true

module Skelerb
  class Component
    NAME_RE = /\A\w+(\.\w+)*\z/.freeze

    attr_reader :application, :name

    alias app application

    def initialize(application, name, &block)
      self.application = application
      self.name = name
      @block = block
    end

    def object
      @object ||= @block&.call(app, self) or raise 'No object specified'
    end

  private

    def application=(value)
      unless value.is_a? Application
        raise TypeError, "Expected #{Application}, got #{value.class}"
      end

      @application = value
    end

    def name=(value)
      value = String(value).to_sym
      raise "Invalid value: #{value.inspect}" unless NAME_RE.match? value

      @name = value
    end
  end
end
