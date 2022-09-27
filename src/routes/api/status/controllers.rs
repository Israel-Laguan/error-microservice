use ::core::panic;
use std::env;

use serde_json::json;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, MiddlewareNext, MiddlewareResult};

// use thruster::ssl_hyper_server::SSLHyperServer;

use deadpool_postgres::{Config, Runtime, Pool};
use tokio_postgres::NoTls;

use crate::cornucopia::queries::create_error::insert_error;
use crate::cornucopia::queries::read_errors::errors;

fn db_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5435);
    cfg.dbname = Some(String::from("postgres"));
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
