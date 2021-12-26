use diesel;
use diesel::prelude::*;
use chrono::{DateTime, NaiveDate, Utc};

use super::schema::metrics;

#[derive(Queryable)]
pub struct Metric {
  pub id: i32,
  pub app_id: i32,
  pub date: NaiveDate,
  pub profile_key: String,
  pub profile_value: String,
  pub count: i32,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "metrics"]
pub struct NewMetric<'a> {
  pub app_id: &'a i32,
  pub date: &'a NaiveDate,
  pub profile_key: &'a str,
  pub profile_value: &'a str,
  pub count: &'a i32,
}

impl Metric {
  pub fn create(new_metric: &NewMetric, conn: &diesel::PgConnection) -> Self {
    diesel::insert_into(metrics::table)
      .values(new_metric)
      .get_result(conn)
      .expect("Error saving new metric")
  }
}