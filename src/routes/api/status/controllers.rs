use hyper::header;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, Context, MiddlewareNext, MiddlewareResult};

#[middleware]
pub async fn am_i_up(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.status(200);
    context.set(header::CONTENT_TYPE.as_ref(), "application/text");
    context.body("OK");
    Ok(context)
}
