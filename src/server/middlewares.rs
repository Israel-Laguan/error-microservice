use thruster::errors::ThrusterError;
use thruster::middleware_fn;
use thruster::BasicContext as Ctx;
use thruster::{MiddlewareNext, MiddlewareResult};

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

#[middleware_fn]
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

            context.body(&format!(
                "{{\"message\": \"{}\",\"success\":false}}",
                err.message
            ));
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
    log::trace!("Trace is filtered");

    context.status(status);
    context.body(&format!("{{\"message\": \"{}\",\"success\":false}}", e));

    Ok(context)
}
