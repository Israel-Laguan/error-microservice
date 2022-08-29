use diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::result::Error;
use uuid::Uuid;

use crate::util::db;
use crate::models::errorss::{ Errors, NewErrors };
use crate::schema::errorss;
use crate::schema::errorss::dsl::*;

pub fn create_errors(new_errors: NewErrors) -> Result<Errors, Error> {
  let conn = db::establish_connection();

  let errors = diesel::insert_into(errorss::table)
    .values(&new_errors)
    .get_result(&conn);

  errors
}

pub fn get_errors(identifier: Uuid) -> Result<Errors, Error> {
  let conn = db::establish_connection();

  let errors = errorss.find(identifier)
    .first(&conn);

  errors
}
