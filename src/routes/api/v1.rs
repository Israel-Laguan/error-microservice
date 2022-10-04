use thruster::context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest};
use thruster::App;

mod errors;
use errors::errors_app;

pub fn v1_app() -> App<HyperRequest, Ctx, ()> {
    let errors = errors_app();
    App::<HyperRequest, Ctx, ()>::create(generate_context, ()).router("/errors", errors)
}
