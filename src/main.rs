mod core;
mod routes;

mod server;
use server::configuration::init_env_variables;
use server::{init_app, init_routes, logger, run_server};

fn main() {
    let config = init_env_variables();

    let logger = logger::init_logger();

    let app = init_app();

    let routed_app = init_routes(app);

    run_server(routed_app, &config.host, config.port);
    match logger {
        Ok(logger) => logger.flush(),
        _ => log::trace!("Logger error"),
    }
}
