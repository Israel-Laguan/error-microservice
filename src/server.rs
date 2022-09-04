// use thruster::async_middleware;
// use thruster::{App, BasicContext as Ctx, Request};

pub mod configuration;
pub mod logger;
pub mod middlewares;
// use middlewares::{json_error_handler, profile};

// pub fn init_env_variables() {

// }

// pub fn init_server(is_prod: bool) -> App<Request, Ctx, ()> {
//     let mut app = App::<Request, Ctx, ()>::new_basic()
//         .use_middleware("/", async_middleware!(Ctx, [json_error_handler, profile]));

//     app.connection_timeout = 5000;

//     return app;
// }
