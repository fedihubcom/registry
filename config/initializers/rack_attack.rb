# frozen_string_literal: true

# Be sure to restart your server when you modify this file.

require 'active_support/cache'

Rack::Attack.cache.store = ActiveSupport::Cache::MemoryStore.new

Rack::Attack.throttle 'req/ip', limit: 120, period: 60, &:ip
