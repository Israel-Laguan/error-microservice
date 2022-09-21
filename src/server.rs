use thruster::context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest};
use thruster::hyper_server::HyperServer;
use thruster::{m, App, ThrusterServer};

pub mod configuration;
pub mod logger;
mod env_variables;
mod middlewares;
use middlewares::{cors, helmet, json_error_handler, profile, recommended_headers_https};

pub fn init_app(/*is_prod: bool*/) -> App<HyperRequest, Ctx, ()> {
    App::<HyperRequest, Ctx, ()>::create(generate_context, ())
        .middleware("/", m![json_error_handler, helmet, recommended_headers_https, profile, cors])
}

pub fn run_server(app: App<HyperRequest, Ctx, ()>, host: &str, port: u16) {
    let server = HyperServer::new(app);
    server.start(host, port);
}
