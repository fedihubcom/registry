# frozen_string_literal: true

# Be sure to restart your server when you modify this file.

Skelerb.app[:rom] do |app, component|
  configuration = app[:database].object

  configuration.auto_registration(
    app.config.root.join('app'),
    namespace: 'Fedihub::Registry',
  )

  ROM.container configuration
end
