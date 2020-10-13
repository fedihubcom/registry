# frozen_string_literal: true

module Skelerb
  def self.application
    @application ||= Application.new
  end
end
