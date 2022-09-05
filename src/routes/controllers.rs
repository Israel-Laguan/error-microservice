use hyper::Body;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, MiddlewareNext, MiddlewareResult};

#[middleware]
pub async fn four_oh_four(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.status(404);
    context.body = Body::from("Whoops! That route doesn't exist!");
    Ok(context)
}

#[middleware]
pub async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Hello, World!";
    context.body = Body::from(val);
    log::info!("value received: {}", val);
    log::trace!("Trace is filtered");
    Ok(context)
}
