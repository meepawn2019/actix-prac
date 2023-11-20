use actix_web::web;

use crate::api::users::lib::{get_users, create_user, login};


pub fn users_config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/users")
      .route(web::get().to(get_users))
      .route(web::post().to(create_user))
  ).service(
    web::resource("/auth/login")
      .route(web::post().to(login))
  );
}