use thruster::context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest};
use thruster::hyper_server::HyperServer;
use thruster::{m, App, ThrusterServer};

pub mod configuration;
pub mod logger;

mod env_variables;
mod middleware;
use middleware::headers::{cors, helmet, recommended_headers_https};
use middleware::profile::trace_time_of_request;
use middleware::validators::json_error_handler;

pub fn init_app(/*is_prod: bool*/) -> App<HyperRequest, Ctx, ()> {
    App::<HyperRequest, Ctx, ()>::create(generate_context, ()).middleware(
        "/",
        m![
            json_error_handler,
            helmet,
            recommended_headers_https,
            trace_time_of_request,
            cors
        ],
    )
}

pub fn run_server(app: App<HyperRequest, Ctx, ()>, host: &str, port: u16) {
    let server = HyperServer::new(app);
    server.start(host, port);
}
