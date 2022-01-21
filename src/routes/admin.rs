use actix_identity::Identity;
use actix_web::{get, web, Error, Responder, HttpResponse};
use serde::Serialize;

#[get("/admin*")]
pub async fn admin(
  id: Identity,
  tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
  match id.identity().as_ref() {
    Some(identity) => {
      let mut ctx = tera::Context::new();
      let s = tmpl.render("admin.html", &ctx)
        .map_err(|e| {
          eprintln!("{}", e);
          HttpResponse::InternalServerError().finish()
        })?;

      Ok(HttpResponse::Ok().body(s))
    }
    _ => Ok(HttpResponse::Found().header("Location", "/login").finish()),
  }
}