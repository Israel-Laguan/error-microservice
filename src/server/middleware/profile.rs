use std::time::Instant;

use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, MiddlewareNext, MiddlewareResult};

#[middleware]
pub async fn trace_time_of_request (mut context: Ctx, next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let start_time = Instant::now();

    context = next(context).await?;

    let elapsed_time = start_time.elapsed();

    match &context.hyper_request {
        Some(hyper_request) => {
            log::info!(
                "[{}μs] {} {}",
                elapsed_time.as_micros(),
                hyper_request.request.method(),
                hyper_request.request.uri(),
            );
        }
        None => panic!("problem"),
    }

    Ok(context)
}
