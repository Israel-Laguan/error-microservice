use thruster::context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest};
use thruster::hyper_server::HyperServer;
// use thruster::ssl_hyper_server::SSLHyperServer;
use thruster::{m, App, ThrusterServer};

pub mod configuration;
pub mod logger;

mod env_variables;
mod middleware;
use middleware::headers::{cors, helmet, recommended_headers_https};
use middleware::profile::trace_time_of_request;
use middleware::validators::json_error_handler;

pub fn init_app(is_prod: Option<bool>) -> App<HyperRequest, Ctx, ()> {
    let app = App::<HyperRequest, Ctx, ()>::create(generate_context, ());
    match is_prod {
        Some(_is_prod) => app.middleware("/", m![json_error_handler, helmet, recommended_headers_https, cors]),
        None => app.middleware("/", m![json_error_handler, trace_time_of_request,]),
    }
}

pub fn run_server(app: App<HyperRequest, Ctx, ()>, host: &str, port: u16, is_prod: Option<bool>) {
    match is_prod {
        Some(_is_prod) => {
            HyperServer::new(app).start(host, port)
        }
        None => HyperServer::new(app).start(host, port),
    }
}
