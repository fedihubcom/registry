# frozen_string_literal: true

# Be sure to restart your server when you modify this file.

Skelerb.app[:sinatra] do |app, component|
  Class.new Sinatra::Application do
    set :root, app.config.root
    set :environment, app.config.env

    get '/' do
      erb :'home/index.html'
    end
  end
end
