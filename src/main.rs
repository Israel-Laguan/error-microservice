mod core;
mod routes;
use routes::init_routes;

mod server;
use server::configuration::init_env_variables;
use server::logger::init_logger;
use server::{init_app, run_server};

mod cornucopia;

fn main() {
    let config = init_env_variables();
    let is_prod = Some(false); //config.env == "PRODUCTION");

    let logger = init_logger();

    let app = init_app(is_prod);

    let routed_app = init_routes(app);

    run_server(routed_app, &config.host, config.port, is_prod);
    match logger {
        Ok(logger) => logger.flush(),
        _ => log::trace!("Logger error"),
    }
}
