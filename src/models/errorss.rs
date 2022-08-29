use uuid::Uuid;
use crate::schema::errorss;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Errors {
  pub id: Uuid,
  pub some_field: Option<String>
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name="errorss"]
pub struct NewErrors<'a> {
  pub some_field: &'a str
}
