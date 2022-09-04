use thruster::async_middleware;
use thruster::{App, BasicContext as Ctx, Request, Server, ThrusterServer};

mod core;

mod routes;
use routes::controllers::{four_oh_four, plaintext};

mod server;
use server::configuration::init_env_variables;
use server::logger;
use server::middlewares::json_error_handler;

fn main() {
    let config = init_env_variables();

    print!("env {:#?}", config.env);
    let logger = logger::init_logger();

    let mut app = App::<Request, Ctx, ()>::new_basic()
        .use_middleware("/", async_middleware!(Ctx, [json_error_handler]))
        .get("/hello", async_middleware!(Ctx, [plaintext]))
        .set404(async_middleware!(Ctx, [four_oh_four]));

    app.connection_timeout = 5000;

    let server = Server::new(app);
    server.start("0.0.0.0", 4321);
    match logger {
        Ok(logger) => logger.flush(),
        _ => log::trace!("Logger error"),
    }
}
