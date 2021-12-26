use diesel;
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize};

use super::schema::versions;

#[derive(Queryable, Serialize)]
pub struct Version {
  pub id: i32,
  pub app_id: i32,
  pub version: String,
  pub min_system_version: String,
  pub description: String,
  pub url: String,
  pub dsa_signature: Option<String>,
  pub ed_signature: Option<String>,
  pub length: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "versions"]
pub struct NewVersion<'a> {
  pub app_id: &'a i32,
  pub version: &'a str,
  pub min_system_version: &'a str,
  pub description: &'a str,
  pub url: &'a str,
  pub dsa_signature: Option<&'a str>,
  pub ed_signature: Option<&'a str>,
  pub length: &'a str,
}

impl Version {
  pub fn create(new_version: NewVersion, conn: &diesel::PgConnection) -> Self {
    diesel::insert_into(versions::table)
      .values(&new_version)
      .get_result(conn)
      .expect("Error saving new post")
  }

  pub fn by_app_id(app_id: &i32, conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
    versions::table.filter(versions::app_id.eq(app_id)).load(conn)
  }
}