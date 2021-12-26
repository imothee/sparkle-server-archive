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
}