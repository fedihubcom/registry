# frozen_string_literal: true

# Be sure to restart your server when you modify this file.

Skelerb.app[:rack] do |app, component|
  Rack::Builder.new do
    use Rack::Attack

    run ->(_) { [200, { 'Content-Type' => 'text/plain' }, ['Hello, World!']] }
  end
end
