use ::core::panic;
use std::env;

use serde_json::json;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, MiddlewareNext, MiddlewareResult};

use deadpool_postgres::{Config, Runtime, Pool};
use tokio_postgres::NoTls;

use crate::cornucopia::queries::create_error::insert_error;
use crate::cornucopia::queries::read_errors::errors;
use crate::server::configuration::env_variables;

fn db_pool() -> Pool {
    let conf = env_variables();
    let mut cfg = Config::new();
    log::error!("{}", conf.postgres_db_url);
    cfg.user = Some(conf.postgres_user);
    cfg.password = Some(conf.postgres_password);
    cfg.host = Some(conf.postgres_db_url);
    cfg.port = Some(5432);
    cfg.dbname = Some(conf.postgres_db);
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls);
    match pool {
        Ok(pool) => pool,
        Err(_) => panic!("expr")
    }
}

#[middleware]
pub async fn about(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    const ID: &str = env!("CARGO_PKG_NAME");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let response: &str = &json!({
        "id": ID,
        "description": DESCRIPTION,
        "version": VERSION,
    })
    .to_string();
    context.status(200);
    context.content_type("application/json");
    context.body(response);
    Ok(context)
}

#[middleware]
pub async fn am_i_up(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.status(200);
    context.content_type("application/text");
    context.body("OK");
    Ok(context)
}

#[middleware]
pub async fn dependency(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let dependency = context.hyper_request.as_ref().unwrap().params.get("dependency").unwrap().param.clone();
    log::info!("{}", dependency);
    let pool = db_pool();
    let client = pool.get().await.unwrap();
    insert_error()
    .bind(&client, &"123", &"abc", &"message", &"location", &"context", &"trace")
    .await
    .unwrap();

    let errors = errors()
    .bind(&client, &"123")
    .map(|error| format!("{}", error))
    .all().await.unwrap();

    let response: &str = &json!({
        "errors": errors,
    }).to_string();

    context.status(200);
    context.content_type("application/json");
    context.body(response);
    Ok(context)
}
