use diesel;
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize};

use super::schema::users;

#[derive(Queryable)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub password_token: String,
  pub last_login: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub email: &'a str,
  pub password_token: &'a str,
}

impl User {
  pub fn create(new_user: &NewUser, conn: &diesel::PgConnection) -> Self {
    diesel::insert_into(users::table)
      .values(new_user)
      .get_result(conn)
      .expect("Error saving new user")
  }

  pub fn get_by_email(email: &str, conn: &diesel::PgConnection) -> QueryResult<Self> {
    users::table
      .filter(users::email.eq(email))
      .first(conn)
  }
}