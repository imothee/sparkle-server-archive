use actix_web::{get, post, web, Error, FromRequest, HttpResponse};
use actix_identity::Identity;
use serde::Deserialize;
use bcrypt::{verify};

use crate::db;
use crate::models::user::{NewUser, User};

#[derive(Deserialize)]
pub struct LoginData {
  email: String,
  password: String,
}

#[get("/login")]
pub async fn login(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
  let mut ctx = tera::Context::new();
  ctx.insert("email", &"");
  ctx.insert("password", &"");
  ctx.insert("error", &false);
  let s = tmpl.render("login.html", &ctx)
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().body(s))
}

#[post("/login")]
pub async fn login_post(
  pool: web::Data<db::DbPool>,
  id: Identity,
  form: web::Form<LoginData>,
  tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {

  let conn = pool.get().expect("couldn't get db connection from pool");
  let user = User::get_by_email(&form.email, &conn);

  let user_id: Option<i32> = match user {
    Ok(user) => match verify(&form.password, &user.password_token) {
      Ok(true) => Some(user.id),
      Ok(false) => None,
      Err(e) => None,
    },
    Err(e) => None,
  };

  if let Some(user_id) = user_id {
    id.remember(user_id.to_string());
    return Ok(HttpResponse::Found().header("Location", "/admin").finish())
  }

  let mut ctx = tera::Context::new();
  ctx.insert("email", &form.email);
  ctx.insert("password", &form.password);
  ctx.insert("error", &true);
  let s = tmpl.render("login.html", &ctx)
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().body(s))
}