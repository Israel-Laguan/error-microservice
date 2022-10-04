use thruster::context::basic_hyper_context::{BasicHyperContext as Ctx, HyperRequest};
use thruster::{m, App};

mod api;
use api::status::status_app;
use api::v1::errors_app;

mod controllers;
use controllers::four_oh_four;

pub fn init_routes(app: App<HyperRequest, Ctx, ()>) -> App<HyperRequest, Ctx, ()> {
    let status = status_app();
    let errors = errors_app();

    app.router("/status", status).router("/errors", errors).set404(m![four_oh_four])
}
