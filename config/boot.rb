# frozen_string_literal: true

Warning[:deprecated] = false

ENV['BUNDLE_GEMFILE'] ||= File.expand_path('../Gemfile', __dir__).freeze

require 'bundler/setup' # Set up gems listed in the Gemfile.

require 'bootsnap'

cache_dir = File.expand_path('../tmp/cache', __dir__).freeze

env = ENV['RACK_ENV']

development_mode = ['', nil, 'development'].include?(env)
test_mode        = env == 'test'

Bootsnap.setup(
  cache_dir: cache_dir,
  development_mode: development_mode,
  load_path_cache: true,
  autoload_paths_cache: true,
  disable_trace: false,
  compile_cache_iseq: test_mode,
  compile_cache_yaml: true,
)
