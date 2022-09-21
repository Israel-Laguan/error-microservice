mod core;
mod routes;
use routes::init_routes;
mod server;
use server::configuration::init_env_variables;
use server::logger::init_logger;
use server::{init_app, run_server};

fn main() {
    let config = init_env_variables();

    let logger = init_logger();

    let app = init_app();

    let routed_app = init_routes(app);

    run_server(routed_app, &config.host, config.port);
    match logger {
        Ok(logger) => logger.flush(),
        _ => log::trace!("Logger error"),
    }
}
