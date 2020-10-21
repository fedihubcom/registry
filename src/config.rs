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
    pub secret_key: Option<String>,
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
        let default_root = match Self::default_root() {
            Err(_) => return Err(()),
            Ok(value) => value,
        };

        Ok(
            Config {
                root: default_root,
                environment: DEFAULT_ENVIRONMENT,
                address: DEFAULT_ADDRESS.to_string(),
                port: DEFAULT_PORT,
                database_url: DEFAULT_DATABASE_URL.to_string(),
                secret_key: None,
            }
         )
    }

    pub fn from_env() -> Result<Self, ()> {
        let mut result = Self::default()?;
        result.use_env();
        Ok(result)
    }

    pub fn default_root() -> Result<String, ()> {
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

    pub fn use_env(&mut self) {
        self.use_env_for_root();
        self.use_env_for_environment();
        self.use_env_for_address();
        self.use_env_for_port();
        self.use_env_for_database_url();
        self.use_env_for_secret_key();
    }

    pub fn to_rocket_config(&self) -> Result<RocketConfig, ()> {
        match self.to_rocket_config_builder().finalize() {
            Err(_) => Err(()),
            Ok(rocket_config) => Ok(rocket_config),
        }
    }

    pub fn to_rocket_config_builder(&self) -> RocketConfigBuilder {
        let mut value =
            RocketConfig::build(self.environment.to_rocket_environment())
            .root(self.root.to_string())
            .address(self.address.to_string())
            .port(self.port);

        if let Some(secret_key) = &self.secret_key {
            value = value.secret_key(secret_key);
        }

        value
    }

    pub fn public_path(&self) -> Result<String, ()> {
        let mut result_path_buf = std::path::PathBuf::new();
        result_path_buf.push(self.root.to_string());
        result_path_buf.push("public");

        let result_str = match result_path_buf.to_str() {
            None => return Err(()),
            Some(value) => value,
        };

        Ok(result_str.to_string())
    }

    pub fn locales_path(&self) -> Result<String, ()> {
        let mut result_path_buf = std::path::PathBuf::new();
        result_path_buf.push(self.root.to_string());
        result_path_buf.push("locales");

        let result_str = match result_path_buf.to_str() {
            None => return Err(()),
            Some(value) => value,
        };

        Ok(result_str.to_string())
    }

    pub fn use_env_for_root(&mut self) {
        self.root = match std::env::var("ROOT") {
            Err(_) => return,
            Ok(value) =>
                if value.is_empty() {
                    return
                }
                else {
                    value
                },
        };
    }

    pub fn use_env_for_environment(&mut self) {
        self.environment = match std::env::var("ENVIRONMENT") {
            Err(_) => return,
            Ok(value) =>
                if value.is_empty() {
                    return
                }
                else {
                    Environment::from_string(value)
                },
        };
    }

    pub fn use_env_for_address(&mut self) {
        self.address = match std::env::var("ADDRESS") {
            Err(_) => return,
            Ok(value) =>
                if value.is_empty() {
                    return
                }
                else {
                    value
                },
        };
    }

    pub fn use_env_for_port(&mut self) {
        self.port = match std::env::var("PORT") {
            Err(_) => return,
            Ok(value) => match value.parse::<u16>() {
                Err(_) => return,
                Ok(value) => value,
            },
        };
    }

    pub fn use_env_for_database_url(&mut self) {
        self.database_url = match std::env::var("DATABASE_URL") {
            Err(_) => return,
            Ok(value) =>
                if value.is_empty() {
                    return
                }
                else {
                    value
                },
        };
    }

    pub fn use_env_for_secret_key(&mut self) {
        self.secret_key = match std::env::var("SECRET_KEY") {
            Err(_) => return,
            Ok(value) =>
                if value.is_empty() {
                    return
                }
                else {
                    Some(value)
                },
        }
    }
}
