use diesel;
use diesel::prelude::*;
use chrono::{DateTime, Utc};

use super::schema::system_profiles;

#[derive(Queryable)]
pub struct SystemProfile {
  pub id: i32,
  pub app_id: i32,
  pub app_version: Option<String>,
  pub cpu64bit: Option<bool>,
  pub ncpu: Option<i32>,
  pub cpu_freq_mhz: Option<String>,
  pub cputype: Option<String>,
  pub cpusubtype: Option<String>,
  pub model: Option<String>,
  pub ram_mb: Option<String>,
  pub os_version: Option<String>,
  pub lang: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "system_profiles"]
pub struct NewSystemProfile<'a> {
  pub app_id: &'a i32,
  pub app_version: Option<&'a str>,
  pub cpu64bit: Option<&'a bool>,
  pub ncpu: Option<&'a i32>,
  pub cpu_freq_mhz: Option<&'a str>,
  pub cputype: Option<&'a str>,
  pub cpusubtype: Option<&'a str>,
  pub model: Option<&'a str>,
  pub ram_mb: Option<&'a str>,
  pub os_version: Option<&'a str>,
  pub lang: Option<&'a str>,
}

impl SystemProfile {
  pub fn create(new_system_profile: &NewSystemProfile, conn: &diesel::PgConnection) -> Self {
    diesel::insert_into(system_profiles::table)
      .values(new_system_profile)
      .get_result(conn)
      .expect("Error saving new profile")
  }

  pub fn all(conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
    system_profiles::table.load(conn)
  }

  pub fn by_app_id(app_id: &i32, conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
    system_profiles::table
      .filter(system_profiles::app_id.eq(app_id))
      .load(conn)
  }

  pub fn by_app_id_between(app_id: &i32, start: &DateTime<Utc>, end: &DateTime<Utc>, conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
    system_profiles::table
      .filter(system_profiles::app_id.eq(app_id))
      .filter(system_profiles::created_at.between(start, end))
      .load(conn)
  }

  pub fn delete_older_than(date: &DateTime<Utc>, conn: &diesel::PgConnection) -> usize {
    diesel::delete(
      system_profiles::table.filter(system_profiles::created_at.lt(date))
    )
      .execute(conn)
      .expect("Error deleting profiles")
  }
}