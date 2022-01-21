use diesel;
use diesel::prelude::*;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Serialize};
use super::schema::metrics;

#[derive(Queryable, Serialize)]
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

#[derive(Insertable,AsChangeset)]
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

  pub fn upsert(new_metric: &NewMetric, conn: &diesel::PgConnection) -> Self {
    diesel::insert_into(metrics::table)
      .values(new_metric)
      .on_conflict((metrics::app_id, metrics::date, metrics::profile_key, metrics::profile_value))
      .do_update()
      .set(new_metric)
      .get_result(conn)
      .expect("Error saving new metric")
  }

  pub fn by_app_id(app_id: &i32, conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
    metrics::table
      .filter(metrics::app_id.eq(app_id))
      .load(conn)
  }

  pub fn by_app_id_date(app_id: &i32, date: &NaiveDate, conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
    metrics::table
      .filter(metrics::app_id.eq(app_id))
      .filter(metrics::date.ge(date))
      .load(conn)
  }
}