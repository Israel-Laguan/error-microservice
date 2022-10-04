use std::env;

use serde_json::json;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, MiddlewareNext, MiddlewareResult};

use crate::core::database::check_db_is_reachable;

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

    match dependency.as_str() {
        "db" => {
            let db_status: bool = check_db_is_reachable().await;
            let response = &json!({
                "ok": db_status.to_string(),
                "message": "Database state was check",
                "db": db_status.to_string()
            })
            .to_string();

            context.status(200);
            context.content_type("application/json");
            context.body(response);
        }
        _ => {
            let response = &json!({
                "ok": "false",
            })
            .to_string();

            context.status(401);
            context.content_type("application/json");
            context.body(response);
        }
    };

    Ok(context)
}
