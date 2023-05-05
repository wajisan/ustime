use serde::Deserialize;
use config::{ConfigError, Environment};
use std::fmt;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}

impl fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "host:{}, port:{}", self.host, self.port)
    }
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub name: String,
    pub host: String,
    pub user: String,
    pub password: String
}

impl fmt::Debug for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "host:{}, user:{}, password:{}, name:{}", self.host, self.user, self.password, self.name)
    }
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig
}


impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let builder = config::Config::builder()
            .add_source(Environment::default());
        match builder.build() {
            Ok(config) => {
                let app_config = AppConfig {
                    server: config.get::<ServerConfig>("server").unwrap(),
                    database: config.get::<DatabaseConfig>("database").unwrap()
                };
                Ok(app_config)
            },
            Err(e) => {
                println!("Oh no");
                Err(e.into())
            }
        }
    }
}