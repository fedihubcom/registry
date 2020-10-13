# frozen_string_literal: true

# Be sure to restart your server when you modify this file.

Skelerb.app[:sinatra] do |app, component|
  Class.new Sinatra::Application do
    set :root, app.config.root
    set :environment, app.config.env

    set :erb,
        layout: :'site.html',
        layout_options: { views: app.config.root.join('views/layouts') }

    get '/' do
      rom = app[:database].object

      user_repo = Fedihub::Registry::Repos::User.new rom

      @users = user_repo.users

      erb :'home/index.html'
    end
  end
end
