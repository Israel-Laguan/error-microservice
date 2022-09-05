use thruster::hyper_server::HyperServer;
use thruster::{m, async_middleware};
use thruster::context::basic_hyper_context::{
    generate_context, BasicHyperContext as Ctx, HyperRequest,
};
use thruster::{App, ThrusterServer};

mod core;

mod routes;
use routes::controllers::{four_oh_four, plaintext};

mod server;
use server::configuration::init_env_variables;
use server::logger;
use server::middlewares::{json_error_handler, profile};

fn main() {
    let config = init_env_variables();

    print!("env {:#?}", config.env);
    let logger = logger::init_logger();

    let mut app = App::<HyperRequest, Ctx, ()>::create(generate_context, ())
        .use_middleware("/", async_middleware!(Ctx, [profile, json_error_handler]))
        .get("/hello", m![plaintext])
        .set404(m![four_oh_four]);

    app.connection_timeout = 5000;

    let server = HyperServer::new(app);
    let host = match std::env::var("HOST") {
        Ok(value) => value,
        Err(e) => panic!("couldn't interpret HOST: {e}"),
    };
    let port = match std::env::var("PORT") {
        Ok(value) => value.parse::<u16>().unwrap(),
        Err(e) => panic!("couldn't interpret PORT: {e}"),
    };
    server.start(&host, port);
    match logger {
        Ok(logger) => logger.flush(),
        _ => log::trace!("Logger error"),
    }
}
