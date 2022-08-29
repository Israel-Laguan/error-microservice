pub mod errors_controller;
pub mod errors_service;

use thruster::{App, middleware, MiddlewareChain, MiddlewareReturnValue, Request};
use crate::context::{generate_context, Ctx};
use crate::errorss::errors_controller::{
  create_errors,
  get_errors,
  update_errors,
  delete_errors
};

pub fn init() -> App<Request, Ctx> {
  let mut subroute = App::<Request, Ctx>::create(generate_context);

  subroute.post("/", middleware![Ctx => create_errors]);
  subroute.get("/:id", middleware![Ctx => get_errors]);
  subroute.put("/:id", middleware![Ctx => update_errors]);
  subroute.delete("/:id", middleware![Ctx => delete_errors]);

  subroute
}
