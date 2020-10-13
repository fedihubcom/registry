# frozen_string_literal: true

module Fedihub
  module Registry
    def self.application
      @application ||= Application.new
    end
  end
end
