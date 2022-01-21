use actix_web::{get, web, Error, Responder, HttpResponse};

#[get("/")]
pub async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
  let mut ctx = tera::Context::new();

  let s = tmpl.render("index.html", &ctx)
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().body(s))
}