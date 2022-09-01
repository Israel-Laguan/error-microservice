extern crate dotenv;

use dotenv::{dotenv, from_filename};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
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
    pub database_url: String,
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

fn default_database_url() -> String {
    "postgres://postgres@localhost/error_microservice".to_string()
}

fn default_log_level() -> String {
    "trace".to_string()
}

fn default_whitelist() -> String {
    "localhost:8080".to_string()
}

pub fn init_env_variables() -> Configuration{
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
        Ok(config) => {
            println!("{:#?}", config);
            config
        },
        Err(error) => panic!("{:#?}", error),
    }
}
