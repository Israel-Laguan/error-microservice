use std::time::Instant;

use hyper::header;
use serde_json::json;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::errors::ThrusterError;
use thruster::{middleware, Context, MiddlewareNext, MiddlewareResult};

use crate::core::validator;

trait ErrorExt {
    fn context(self, context: Ctx) -> ThrusterError<Ctx>;
}

impl<E: Into<validator::Error>> ErrorExt for E {
    fn context(self, context: Ctx) -> ThrusterError<Ctx> {
        ThrusterError {
            context,
            message: "Failed to handle error".to_string(),
            status: 500,
            cause: Some(Box::new(self.into())),
        }
    }
}

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

    log::info!("value received: {}", e);

    let response: &str = &json!({
        "message": err.message,
        "success": false,
    })
    .to_string();

    context.status(status);
    context.body(response);

    Ok(context)
}

#[middleware]
pub async fn profile(mut context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let start_time = Instant::now();

    context = next(context).await?;

    let elapsed_time = start_time.elapsed();

    match &context.hyper_request {
        Some(hyper_request) => {
            log::info!(
                "[{}μs] {} {} {:?}",
                elapsed_time.as_micros(),
                hyper_request.request.method(),
                hyper_request.request.uri(),
                hyper_request.request.headers()
            );
        }
        None => panic!("problem"),
    }

    Ok(context)
}

// following defaults from github.com/helmetjs/helmet
#[middleware]
pub async fn helmet(mut context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.remove(header::SERVER.as_ref());
    context.set(
        header::CONTENT_SECURITY_POLICY.as_ref(),
        "default-src 'self';base-uri 'self';font-src 'self' https: data:;form-action 'self';frame-ancestors \
         'self';img-src 'self' data:;object-src 'none';script-src 'self';script-src-attr 'none';style-src 'self' \
         https: 'unsafe-inline';upgrade-insecure-requests",
    );
    context.set("Cross-Origin-Embedder-Policy", "require-corp");
    context.set("Cross-Origin-Opener-Policy", "same-origin");
    context.set("Cross-Origin-Resource-Policy", "same-origin");
    context.set("Origin-Agent-Cluster", "?1");
    context.set(header::REFERRER_POLICY.as_ref(), "no-referrer");
    context.set(header::STRICT_TRANSPORT_SECURITY.as_ref(), "max-age=15552000; includeSubDomains");
    context.set(header::X_CONTENT_TYPE_OPTIONS.as_ref(), "nosniff");
    context.set(header::X_DNS_PREFETCH_CONTROL.as_ref(), "off");
    context.set("X-Download-Options", "noopen");
    context.set(header::X_FRAME_OPTIONS.as_ref(), "SAMEORIGIN");
    context.set("X-Permitted-Cross-Domain-Policies", "none");
    context.set(header::X_XSS_PROTECTION.as_ref(), "0");

    context = next(context).await?;

    Ok(context)
}

#[middleware]
pub async fn cors(mut context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let origin_env = std::env::var("WHITELIST").unwrap_or_else(|_| "*".to_string());

    let origin = if origin_env.contains(',') {
        let header = context
            .hyper_request
            .as_ref()
            .unwrap()
            .request
            .headers()
            .get("Origin")
            .map(|origin| origin.to_str().unwrap().to_string())
            .unwrap_or_else(|| "*".to_string());

        origin_env.split(',').find(|v| v == &header).unwrap_or("")
    } else {
        &origin_env
    };

    context.set("Access-Control-Allow-Origin", origin);
    context.set("Access-Control-Allow-Headers", "*");
    context.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");

    next(context).await
}
