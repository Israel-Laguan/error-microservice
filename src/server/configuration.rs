extern crate dotenv;

use dotenv::{dotenv, from_filename};
use secrecy::Secret;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    #[serde(default = "default_env")]
    pub env: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_postgres_db")]
    pub postgres_db: String,
    #[serde(default = "default_postgres_user")]
    pub postgres_user: String,
    #[serde(default = "default_postgres_password")]
    pub postgres_password: String,
    #[serde(default = "default_postgres_db_url")]
    pub postgres_db_url: String,
    #[serde(default = "default_database_url")]
    pub database_url: Secret<String>,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_whitelist")]
    pub whitelist: String,
}

fn default_env() -> String {
    "LOCAL".to_string()
}

fn default_host() -> String {
    "0.0.0 .0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_postgres_db() -> String {
    "error_microservice".to_string()
}

fn default_postgres_user() -> String {
    "postgres".to_string()
}

fn default_postgres_password() -> String {
    "$3cr3+".to_string()
}

fn default_postgres_db_url() -> String {
    "0.0.0 .0".to_string()
}

fn default_database_url() -> Secret<String> {
    let url = "postgres://postgres@localhost/error_microservice".to_string();
    Secret::new(url)
}

fn default_log_level() -> String {
    "trace".to_string()
}

fn default_whitelist() -> String {
    "localhost:8080".to_string()
}

pub fn init_env_variables() -> Configuration {
    let env = match env::var_os("ENV") {
        Some(value) => value.into_string().unwrap(),
        None => "LOCAL".to_string(),
    };

    let env_file = from_filename(format!(".env.{}", env.to_lowercase())).ok();

    match env_file {
        Some(_) => env_file,
        _ => dotenv().ok(),
    };
    match envy::from_env::<Configuration>() {
        // if we could load the config using the existing env variables - use that
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    }
}
