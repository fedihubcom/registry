use rocket::config::{
    Config as RocketConfig,
    ConfigBuilder as RocketConfigBuilder,
    Environment as RocketEnvironment,
};

const DEFAULT_ENVIRONMENT: Environment = Environment::Development;
const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8000;

pub enum Environment {
    Development,
    Test,
    Production,
}

pub struct Config {
    root: String,
    environment: Environment,
    address: String,
    port: u16,
}

impl Environment {
    pub fn to_rocket_environment(&self) -> RocketEnvironment {
        match self {
            Environment::Development => RocketEnvironment::Development,
            Environment::Test        => RocketEnvironment::Development,
            Environment::Production  => RocketEnvironment::Production,
        }
    }
}

impl Config {
    pub fn default() -> Result<Self, ()> {
        let root_path_buf = match std::env::current_dir() {
            Err(_) => return Err(()),
            Ok(value) => value,
        };

        let root_str = match root_path_buf.to_str() {
            None => return Err(()),
            Some(value) => value,
        };

        let root_string = root_str.to_string();

        Ok(
            Config {
                root: root_string,
                environment: DEFAULT_ENVIRONMENT,
                address: DEFAULT_ADDRESS.to_string(),
                port: DEFAULT_PORT,
            }
         )
    }

    pub fn to_rocket_config_builder(&self) -> RocketConfigBuilder {
        RocketConfig::build(self.environment.to_rocket_environment())
            .root(self.root.to_string())
            .address(self.address.to_string())
            .port(self.port)
    }
}
