use thruster::context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest};
use thruster::hyper_server::HyperServer;
use thruster::{m, App, ThrusterServer};

pub mod configuration;
pub mod logger;
mod middlewares;
use middlewares::{cors, helmet, json_error_handler, profile};

use crate::routes::api::status::status_app;
use crate::routes::controllers::{four_oh_four, plaintext};

pub fn init_app(/*is_prod: bool*/) -> App<HyperRequest, Ctx, ()> {
    App::<HyperRequest, Ctx, ()>::create(generate_context, ())
        .middleware("/", m![json_error_handler, helmet, profile, cors])
}

pub fn init_routes(app: App<HyperRequest, Ctx, ()>) -> App<HyperRequest, Ctx, ()> {
    let status = status_app();

    app.get("/hello", m![plaintext]).router("/status", status).set404(m![four_oh_four])
}

pub fn run_server(app: App<HyperRequest, Ctx, ()>, host: &str, port: u16) {
    let server = HyperServer::new(app);
    server.start(host, port);
}
