use bcrypt::{DEFAULT_COST, hash};
use std::io::{stdin,stdout,Write};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use sparkle_server::models::user::{User, NewUser};

fn main() {
  let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<PgConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");

  let email = getInput("email address");
  let password = getInput("password");

  let hashed = hash(password, DEFAULT_COST).expect("Failed to has password");

  let new_user = NewUser {
    email: &email,
    password_token: &hashed,
  };

  let conn = pool.get().expect("couldn't get db connection from pool");

  let user = User::create(&new_user, &conn);
  print!("Created user with id {} and email {}", user.id, user.email);
}

fn getInput(prompt: &str) -> String {
  let mut s= String::new();
  print!("Please enter {}", prompt);
  let _=stdout().flush();
  stdin().read_line(&mut s).expect("Did not enter a correct string");
  if let Some('\n')=s.chars().next_back() {
    s.pop();
  }
  if let Some('\r')=s.chars().next_back() {
    s.pop();
  }
  s
}