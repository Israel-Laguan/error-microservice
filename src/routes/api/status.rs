use thruster::context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest};
use thruster::{m, App};

mod controllers;

use controllers::am_i_up;

pub fn status_app() -> App<HyperRequest, Ctx, ()> {
    App::<HyperRequest, Ctx, ()>::create(generate_context, ()).get("/am-i-up", m![am_i_up])
}
