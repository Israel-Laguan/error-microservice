use serde_json::json;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, MiddlewareNext, MiddlewareResult};

use crate::core::validator;

#[middleware]
pub async fn json_error_handler(context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let res = next(context).await;

    // If there is not an error, return
    let mut err = match res {
        Ok(_) => return res,
        Err(err) => err,
    };

    // Generic handler, if we fail to downcast
    let e: Box<validator::Error> = match err.cause.take().map(|cause| cause.downcast()) {
        Some(Ok(e)) => e,
        _ => {
            let mut context = err.context;

            let response: &str = &json!({
                "message": err.message,
                "success": false,
            })
            .to_string();

            context.body(response);
            context.status(err.status);

            return Ok(context);
        }
    };

    // Handle the Error variants
    let mut context = err.context;
    let status = match *e {
        validator::Error::InvalidId { .. } => 400,
        validator::Error::FileNotFound { .. } => 404,
    };

    let response: &str = &json!({
        "message": err.message,
        "success": false,
    })
    .to_string();

    context.status(status);
    context.body(response);

    Ok(context)
}
