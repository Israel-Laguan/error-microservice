use crate::cornucopia::queries::create_error::insert_error;
use crate::cornucopia::queries::read_errors::errors;

pub fn insert() {
    let pool = db_pool();
    let client = pool.get().await.unwrap();
    insert_error()
    .bind(&client, &"123", &"abc", &"message", &"location", &"context", &"trace")
    .await
    .unwrap();
}

pub fn list() {
    let errors = errors()
    .bind(&client, &"123")
    .map(|error| format!("{}", error))
    .all().await.unwrap();
}
