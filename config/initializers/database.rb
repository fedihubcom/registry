# frozen_string_literal: true

# Be sure to restart your server when you modify this file.

Skelerb.app[:database] do |app, component|
  dbname = "fedihub_#{app.config.environment}"
  credentials = "postgres://fedihub:fedihub@localhost:5432/#{dbname}"

  configuration = ROM::Configuration.new :sql, credentials
  configuration.auto_registration(
    app.config.root.join('app'),
    namespace: 'Fedihub::Registry',
  )

  ROM.container configuration
end
