# frozen_string_literal: true

Warning[:deprecated] = false

ENV['BUNDLE_GEMFILE'] ||= File.expand_path('../Gemfile', __dir__).freeze

require 'bundler/setup' # Set up gems listed in the Gemfile.

require 'bootsnap'

cache_dir = File.expand_path('../tmp/cache', __dir__).freeze

env = String(ENV['RACK_ENV']).strip.freeze
env = nil if env.empty?
env ||= 'development'

ENV['RACK_ENV'] = ENV['APP_ENV'] = ENV['ENV'] = env

Bootsnap.setup(
  cache_dir: cache_dir,
  development_mode: env == 'development',
  load_path_cache: true,
  autoload_paths_cache: true,
  disable_trace: false,
  compile_cache_iseq: env == 'test',
  compile_cache_yaml: true,
)
