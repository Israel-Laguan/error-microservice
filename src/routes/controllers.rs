use thruster::{middleware_fn};
use thruster::{BasicContext as Ctx};
use thruster::{MiddlewareNext, MiddlewareResult};

#[middleware_fn]
pub async fn four_oh_four(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.status(404);
    context.body("Whoops! That route doesn't exist!");
    Ok(context)
}

#[middleware_fn]
pub async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Hello, World!";
    context.body(val);
    log::info!("value received: {}", val);
    log::trace!("Trace is filtered");
    Ok(context)
}
