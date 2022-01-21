use actix_identity::Identity;
use actix_web::{get, web, Error, Responder, HttpResponse};
use chrono::{Duration, NaiveDate, Utc, Weekday};
use serde::{Serialize, Deserialize};
use std::ops::{Sub};

use crate::db;
use crate::models::app::App;
use crate::models::metric::Metric;

#[derive(Serialize)]
struct AppsResponse {
  apps: Vec<App>
}

#[derive(Serialize)]
struct MetricsResponse {
  metrics: Vec<Metric>
}

#[derive(Deserialize)]
struct AppInfo {
  app_id: i32,
}

#[derive(Deserialize)]
struct MetricQuery {
  date: Option<NaiveDate>,
  profile_key: Option<String>,
}

#[get("/api/apps")]
pub async fn api_apps_index(
  id: Identity,
  pool: web::Data<db::DbPool>,
) -> Result<HttpResponse, Error> {
  match id.identity().as_ref() {
    Some(identity) => {
      let conn = pool.get().expect("couldn't get db connection from pool");
      let apps = App::all(&conn)
        .map_err(|e| {
          eprintln!("{}", e);
          HttpResponse::InternalServerError().finish()
        })?;

      Ok(HttpResponse::Ok().json(&AppsResponse{apps}))
    }
    _ => Ok(HttpResponse::Forbidden().finish()),
  }
}

#[get("/api/apps/{app_id}/metrics")]
pub async fn api_apps_metrics_index(
  id: Identity,
  info: web::Path<AppInfo>,
  query: web::Query<MetricQuery>,
  pool: web::Data<db::DbPool>,
) -> Result<HttpResponse, Error> {
  match id.identity().as_ref() {
    Some(identity) => {
      let conn = pool.get().expect("couldn't get db connection from pool");

      let date = match query.date {
        Some(d) => d,
        None => Utc::now().sub(Duration::weeks(8 as i64)).date().naive_utc(),
      };

      let metrics = Metric::by_app_id_date(&info.app_id, &date, &conn)
        .map_err(|e| {
          eprintln!("{}", e);
          HttpResponse::InternalServerError().finish()
        })?;

      Ok(HttpResponse::Ok().json(&MetricsResponse{metrics}))
    }
    _ => Ok(HttpResponse::Forbidden().finish()),
  }
}