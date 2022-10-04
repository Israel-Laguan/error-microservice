use thruster::context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest};
use thruster::{m, App};

mod errors;

use errors::list;

pub fn errors_app() -> App<HyperRequest, Ctx, ()> {
    App::<HyperRequest, Ctx, ()>::create(generate_context, ()).get("/", m![list])
}
