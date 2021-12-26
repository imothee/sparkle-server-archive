use diesel;
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize};

use super::schema::apps;

#[derive(Queryable, Serialize)]
pub struct App {
  pub id: i32,
  pub name: String,
  pub slug: String,
  pub descriptiom: String,
  pub icon: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "apps"]
pub struct NewApp<'a> {
  pub name: &'a str,
  pub slug: &'a str,
  pub description: &'a str,
  pub icon: Option<&'a str>,
}

impl App {
  pub fn create(new_app: &NewApp, conn: &diesel::PgConnection) -> Self {
    diesel::insert_into(apps::table)
      .values(new_app)
      .get_result(conn)
      .expect("Error saving new app")
  }

  pub fn get_by_slug(slug: &String, conn: &diesel::PgConnection) -> QueryResult<Self> {
    apps::table
      .filter(apps::slug.eq(slug))
      .first(conn)
  }
}