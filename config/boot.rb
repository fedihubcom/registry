# frozen_string_literal: true

Warning[:deprecated] = false

ENV['BUNDLE_GEMFILE'] ||= File.expand_path('../Gemfile', __dir__).freeze

require 'bundler/setup' # Set up gems listed in the Gemfile.

env = String(ENV['RACK_ENV']).strip.freeze
env = nil if env.empty?
env ||= 'development'

ENV['RACK_ENV'] = ENV['APP_ENV'] = ENV['ENV'] = env
