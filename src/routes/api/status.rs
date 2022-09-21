use thruster::context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest};
use thruster::{m, App};

mod controllers;

use controllers::{about, am_i_up, dependency};

pub fn status_app() -> App<HyperRequest, Ctx, ()> {
    App::<HyperRequest, Ctx, ()>::create(generate_context, ())
        .get("/about", m![about])
        .get("/am-i-up", m![am_i_up])
        .get("/:dependency", m![dependency])
}
