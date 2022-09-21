use std::env;

use serde_json::json;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, MiddlewareNext, MiddlewareResult};

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
    context.status(200);
    context.content_type("application/text");
    context.body("OK");
    Ok(context)
}
