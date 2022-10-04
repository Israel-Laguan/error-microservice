use serde_json::json;
use thruster::context::basic_hyper_context::BasicHyperContext as Ctx;
use thruster::{middleware, MiddlewareNext, MiddlewareResult};

use crate::core::database::db_pool;
// use crate::cornucopia::queries::create_error::insert_error;
use crate::cornucopia::queries::read_errors::errors;

#[middleware]
pub async fn list(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let pool = db_pool();
    let client = pool.get().await.unwrap();
    let errors = errors().bind(&client, &"123").map(|error| format!("{}", error)).all().await.unwrap();

    let response = &json!({
        "message": "Database state was check",
        "data": errors

    }).to_string();
    context.status(201);
    context.content_type("application/json");
    context.body(response);
    Ok(context)
}

// #[middleware]
// pub async fn new(mut context: Ctx, _next: MiddlewareNext<Ctx>) ->
// MiddlewareResult<Ctx> {     let resp_body =
// context.hyper_request.unwrap().body;

//     match resp_body {
//         Some(body) => {
//             let pool = db_pool();
//             let client = pool.get().await.unwrap();
//             insert_error()
//             .bind(client, body, body.user_id, body.message, body.location,
// body.context, body.trace).await.unwrap();             let response = &json!({
//                 "message": "Database state was check",
//                 "db": db_status.to_string()
//             });
//             context.status(201);
//             context.content_type("application/json");
//             context.body(response);
//         },
//         None => {

//         }
//     }

//     Ok(context)
// }
