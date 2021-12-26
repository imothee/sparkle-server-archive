#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use tera::{Tera};

mod db;
mod models;
mod routes;

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

  println!("Listening on: {}:{}, open browser and visit have a try!", addr, port);

  // Start HTTP server
  HttpServer::new(move || {
    let tera =
      Tera::new(
        &template_dir
      ).unwrap();

    App::new()
      .data(tera)
      .data(pool.clone())
      .wrap(middleware::Logger::default()) // enable logger
      .service(routes::index)
      .service(routes::get_app_xml)
  })
    .bind((addr, port))?
    .run()
    .await
}