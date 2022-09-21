use thruster::context::basic_hyper_context::{BasicHyperContext as Ctx, HyperRequest};

use thruster::{m, App};

mod api;
use api::status::status_app;

mod controllers;
use controllers::four_oh_four;


pub fn init_routes(app: App<HyperRequest, Ctx, ()>) -> App<HyperRequest, Ctx, ()> {
    let status = status_app();

    app.router("/status", status).set404(m![four_oh_four])
}
