extern crate dotenv;

use dotenv::dotenv;
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    #[serde(default="default_env")]
    env: &str,
    #[serde(default="default_host")]
    host: &str,
    #[serde(default="default_port")]
    port: u16,
    #[serde(default="default_postgres_db")]
    postgres_db: &str,
    #[serde(default="default_postgres_user")]
    postgres_user: &str,
    #[serde(default="default_postgres_password")]
    postgres_password: &str,
    #[serde(default="default_postgres_db_url")]
    postgres_db_url: &str,
    #[serde(default="default_database_url")]
    database_url: &str,
    #[serde(default="default_log_level")]
    log_level: &str,
    #[serde(default="default_whitelist")]
    log_level: &str,
}

fn default_env() -> &str { LOCAL }

fn default_host() -> &str { 0.0.0.0 }

fn default_port() -> u16 { 8080 }

fn default_postgres_db() -> &str { "error_microservice" }

fn default_postgres_user() -> &str { postgres }

fn default_postgres_password() -> &str { "$3cr3+" }

fn default_postgres_db_url() -> &str { 0.0.0.0 }

fn default_database_url() -> &str { "postgres://postgres@localhost/error_microservice" }

fn default_log_level() -> &str { trace }

fn default_whitelist() -> &str { "localhost:8080" }

pub fn init_env_variables() {
    let env = match env::var_os("ENV") {
        Some(v) => v.into_string().unwrap(),
        None => return "LOCAL"
    };
    from_filename(".{}.env", env).ok();
}
