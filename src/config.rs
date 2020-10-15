use rocket::config::{
    Config as RocketConfig,
    ConfigBuilder as RocketConfigBuilder,
    Environment as RocketEnvironment,
};

const DEFAULT_ENVIRONMENT: Environment = Environment::Development;
const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8000;
const DEFAULT_DATABASE_URL: &str =
    "postgres://fedihub:fedihub@localhost/fedihub_development";

#[derive(Debug)]
pub enum Environment {
    Development,
    Test,
    Production,
}

#[derive(Debug)]
pub struct Config {
    pub root: String,
    pub environment: Environment,
    pub address: String,
    pub port: u16,
    pub database_url: String,
}

impl Environment {
    pub fn from_string(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "development" | "dev" => Self::Development,
            "test"                => Self::Test,
            "production" | "prod" => Self::Production,
            _                     => Self::Development,
        }
    }

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
        let root = match current_dir() {
            Err(_) => return Err(()),
            Ok(value) => value,
        };

        Ok(
            Config {
                root,
                environment: DEFAULT_ENVIRONMENT,
                address: DEFAULT_ADDRESS.to_string(),
                port: DEFAULT_PORT,
                database_url: DEFAULT_DATABASE_URL.to_string(),
            }
         )
    }

    pub fn from_env() -> Result<Self, ()> {
        let default_root = match current_dir() {
            Err(_) => return Err(()),
            Ok(value) => value,
        };

        let root = match std::env::var("ROOT") {
            Ok(value) =>
                if value.is_empty() {
                    default_root
                }
                else {
                    value
                },
            Err(error) => match error {
                std::env::VarError::NotPresent => default_root,
                std::env::VarError::NotUnicode(_) => return Err(()),
            },
        };

        let environment = match std::env::var("ENVIRONMENT") {
            Ok(value) => Environment::from_string(value),
            Err(error) => match error {
                std::env::VarError::NotPresent => DEFAULT_ENVIRONMENT,
                std::env::VarError::NotUnicode(_) => return Err(()),
            },
        };

        let address = match std::env::var("ADDRESS") {
            Ok(value) =>
                if value.is_empty() {
                    DEFAULT_ADDRESS.to_string()
                }
                else {
                    value
                },
            Err(error) => match error {
                std::env::VarError::NotPresent => DEFAULT_ADDRESS.to_string(),
                std::env::VarError::NotUnicode(_) => return Err(()),
            },
        };

        let port = match std::env::var("PORT") {
            Ok(value) => match value.parse::<u16>() {
                Ok(value) => value,
                Err(_) => return Err(()),
            },
            Err(error) => match error {
                std::env::VarError::NotPresent => DEFAULT_PORT,
                std::env::VarError::NotUnicode(_) => return Err(()),
            },
        };

        let database_url = match std::env::var("DATABASE_URL") {
            Ok(value) =>
                if value.is_empty() {
                    DEFAULT_DATABASE_URL.to_string()
                }
                else {
                    value
                },
            Err(error) => match error {
                std::env::VarError::NotPresent => DEFAULT_DATABASE_URL.to_string(),
                std::env::VarError::NotUnicode(_) => return Err(()),
            },
        };

        Ok(
            Config {
                root,
                environment,
                address,
                port,
                database_url,
            }
         )
    }

    pub fn to_rocket_config(&self) -> Result<RocketConfig, ()> {
        match self.to_rocket_config_builder().finalize() {
            Err(_) => Err(()),
            Ok(rocket_config) => Ok(rocket_config),
        }
    }

    pub fn to_rocket_config_builder(&self) -> RocketConfigBuilder {
        RocketConfig::build(self.environment.to_rocket_environment())
            .root(self.root.to_string())
            .address(self.address.to_string())
            .port(self.port)
    }
}

fn current_dir() -> Result<String, ()> {
    let root_path_buf = match std::env::current_dir() {
        Err(_) => return Err(()),
        Ok(value) => value,
    };

    let root_str = match root_path_buf.to_str() {
        None => return Err(()),
        Some(value) => value,
    };

    Ok(root_str.to_string())
}
