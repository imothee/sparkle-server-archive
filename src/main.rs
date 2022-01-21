use actix_web::{web, middleware, App, HttpServer};
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use tera::{Tera};

fn assets(cfg: &mut web::ServiceConfig) {
  let env = std::env::var("ENV").unwrap_or_else(|_| "development".to_string());
  if env != "development" {
    cfg.service(fs::Files::new("/assets", "/app/dist").show_files_listing());
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<PgConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");

  let port = std::env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .expect("PORT must be a number");

  let env = std::env::var("ENV").unwrap_or_else(|_| "development".to_string());
  let template_dir = if env == "production" { "/app/templates/**/*" } else { concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*") };
  let addr = if env == "production" { "0.0.0.0" } else { "127.0.0.1" };

  println!("Listening on: {}:{}", addr, port);

  // Start HTTP server
  HttpServer::new(move || {
    let mut tera = Tera::new(&template_dir).unwrap();
    tera.register_function("assets", sparkle_server::middleware::assets);

    App::new()
      .data(tera)
      .data(pool.clone())
      .wrap(middleware::Logger::default())
      .wrap(IdentityService::new(CookieIdentityPolicy::new(&[0; 32])
                                   //.domain("")
                                   .name("auth-cookie")
                                   .path("/")
                                   .secure(false)))
      .configure(assets)// enable logger
      .service(sparkle_server::routes::pages::index)
      .service(sparkle_server::routes::app::get_app_xml)
      .service(sparkle_server::routes::auth::login)
      .service(sparkle_server::routes::auth::login_post)
      .service(sparkle_server::routes::admin::admin)
      .service(sparkle_server::routes::api::api_apps_index)
      .service(sparkle_server::routes::api::api_apps_metrics_index)
  })
    .bind((addr, port))?
    .run()
    .await
}