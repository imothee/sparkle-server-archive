use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use chrono::{Datelike, DateTime, Duration, NaiveDate, TimeZone, Utc, Weekday};
use std::ops::{Add, Sub};

use sparkle_server::models::system_profiles::SystemProfile;

fn main() {
  let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<PgConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");

  let conn = pool.get().expect("couldn't get db connection from pool");

  let days_from_sunday = Utc::now().weekday().num_days_from_sunday();
  let time = Utc::now().sub(Duration::days(days_from_sunday as i64)).sub(Duration::weeks(1 as i64));

  let local_time = NaiveDate::from_ymd(
    time.naive_local().year(),
    time.naive_local().month(),
    time.naive_local().day(),
  ).and_hms(0, 0, 0);
  let date = time.timezone().from_local_datetime(&local_time).unwrap();

  let deleted = SystemProfile::delete_older_than(&date, &conn);
  println!("Deleted {} rows", deleted);
}