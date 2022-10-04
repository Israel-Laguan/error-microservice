use thruster::context::basic_hyper_context::{BasicHyperContext as Ctx, HyperRequest};
use thruster::{m, App};

mod api;
use api::status::status_app;
use api::v1::v1_app;

mod controllers;
use controllers::four_oh_four;

pub fn init_routes(app: App<HyperRequest, Ctx, ()>) -> App<HyperRequest, Ctx, ()> {
    let status = status_app();
    let v1 = v1_app();

    app.router("/status", status).router("/v1", v1).set404(m![four_oh_four])
}
