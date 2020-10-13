# frozen_string_literal: true

# Be sure to restart your server when you modify this file.

Skelerb.app[:database] do |app, component|
  dbname = "fedihub_#{app.config.environment}"
  credentials = "postgres://fedihub:fedihub@localhost:5432/#{dbname}"

  ROM.container :sql, credentials
end
