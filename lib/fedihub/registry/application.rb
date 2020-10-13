# frozen_string_literal: true

module Fedihub
  module Registry
    class Application
      def root
        @root or raise 'No root specified'
      end

      def root=(value)
        @root = Pathname.new(value).realpath.freeze
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
    end
  end
end
