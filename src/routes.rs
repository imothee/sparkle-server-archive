use actix_web::{get, web, Error, Responder, HttpResponse};
use serde::Deserialize;

use crate::db;
use crate::models::app::App;
use crate::models::system_profiles::{SystemProfile, NewSystemProfile};
use crate::models::version::Version;

#[derive(Deserialize)]
pub struct Profiling {
  #[serde(rename(deserialize = "cpu64bit"))]
  cpu64bit: Option<i32>,
  ncpu: Option<i32>,
  #[serde(rename(deserialize = "appVersion"))]
  app_version: Option<String>,
  #[serde(rename(deserialize = "cpuFreqMHz"))]
  cpu_freq_mhz: Option<String>,
  cputype: Option<String>,
  cpusubtype: Option<String>,
  model: Option<String>,
  #[serde(rename(deserialize = "ramMB"))]
  ram_mb: Option<String>,
  #[serde(rename(deserialize = "osVersion"))]
  os_version: Option<String>,
  lang: Option<String>,
}

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[get("/updates/{app_slug}.xml")]
pub async fn get_app_xml(
  pool: web::Data<db::DbPool>,
  tmpl: web::Data<tera::Tera>,
  app_slug: web::Path<String>,
  profile: web::Query<Profiling>,
) -> Result<HttpResponse, Error> {
  let conn = pool.get().expect("couldn't get db connection from pool");
  let app = web::block(move || App::get_by_slug(&app_slug, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  let conn = pool.get().expect("couldn't get db connection from pool");
  let versions = web::block(move || Version::by_app_id(&app.id, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  let conn = pool.get().expect("couldn't get db connection from pool");

  let is64bit = profile.cpu64bit.map(|f| f == 1);
  let profile = NewSystemProfile {
    app_id: &app.id,
    app_version: profile.app_version.as_deref(),
    cpu64bit: is64bit.as_ref(),
    ncpu: profile.ncpu.as_ref(),
    cpu_freq_mhz: profile.cpu_freq_mhz.as_deref(),
    cputype: profile.cputype.as_deref(),
    cpusubtype: profile.cpusubtype.as_deref(),
    model: profile.model.as_deref(),
    ram_mb: profile.ram_mb.as_deref(),
    os_version: profile.os_version.as_deref(),
    lang: profile.lang.as_deref(),
  };

  SystemProfile::create(&profile, &conn);

  let mut ctx = tera::Context::new();
  ctx.insert("app", &app);
  ctx.insert("versions", &versions);

  let s = tmpl.render("appcast.xml", &ctx)
      .map_err(|e| {
          eprintln!("{}", e);
          HttpResponse::InternalServerError().finish()
      })?;
  
  Ok(HttpResponse::Ok().content_type("application/xml").body(s))
}