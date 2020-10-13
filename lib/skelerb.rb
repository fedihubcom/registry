# frozen_string_literal: true

module Skelerb
  class << self
    def application
      @application ||= Application.new
    end

    alias app application
  end
end
