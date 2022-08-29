use crate::context::{ Ctx };
use thruster::{MiddlewareChain, MiddlewareReturnValue};

use crate::errorss::errors_service;
use crate::models::errorss::{ NewErrors, Errors };
use futures::future;
use std::boxed::Box;
use uuid::Uuid;

pub fn create_errors(mut context: Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  match serde_json::from_str::<NewErrors>(&context.request.body()) {
    Ok(new_errors) => {
      match errors_service::create_errors(new_errors) {
        Ok(errors) => {
          context.body(&serde_json::to_string(&errors).unwrap());
        },
        Err(e) => {
          context.status(400);
          context.body("Could not create a new Errors");
        }
      };
    },
    Err(e) => {
      context.status(400);
      context.body("Could not create a new Errors");
    }
  };

  Box::new(future::ok(context))
}

pub fn get_errors(mut context: Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  fn error(mut context: Ctx) -> MiddlewareReturnValue<Ctx> {
    context.status(400);
    context.body("Could not get Errors");
    Box::new(future::ok(context))
  }

  let id = match context.params.get("id") {
    Some(_id) => _id,
    None => return error(context)
  };

  let id_as_number = match id.parse::<i32>() {
    Ok(_id_as_number) => _id_as_number,
    Err(_) => return error(context)
  };

  let fetched_result = match errors_service::get_errors(id_as_number) {
    Ok(_fetched_result) => _fetched_result,
    Err(_) => return error(context)
  };

  match serde_json::to_string(&fetched_result) {
    Ok(result) => context.body(&result),
    Err(_) => return error(context)
  };

  Box::new(future::ok(context))
}

pub fn update_errors(mut context: Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  Box::new(future::ok(context))
}

pub fn delete_errors(mut context:Ctx, _next: impl Fn(Ctx) -> MiddlewareReturnValue<Ctx>  + Send + Sync) -> MiddlewareReturnValue<Ctx> {
  Box::new(future::ok(context))
}
