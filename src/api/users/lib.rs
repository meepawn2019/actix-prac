use actix_web:: {web, Responder, HttpResponse};
use crate::database::lib::connection;
use crate::models::user::{User, NewUser};
use crate::schema::users::username;
use diesel::prelude::*;

pub async fn get_users() -> impl Responder {
  use crate::schema::users::dsl::*;

  let connection = &mut connection();

  let results = users.select(User::as_select()).load(connection).expect("Error getting posts");

  for result in &results {
    println!("User is : {:?}", result)
  }

  let response = serde_json::to_string(&results).unwrap();

  HttpResponse::Ok().body(response)
}

pub async fn create_user(body: web::Json<NewUser>) -> impl Responder {
  use crate::schema::users;

  let connection = &mut connection();

  let new_user: &mut NewUser = &mut body.into_inner();

  let _ = NewUser::hash_password(new_user);

  match diesel::insert_into(users::table)
  .values(&*new_user)
  .returning(User::as_returning())
  .get_result(connection) {
    Ok(result) => {
      let response = serde_json::to_string(&result).unwrap();
      return HttpResponse::Ok().body(response);
    }
    Err(e) => {
      return HttpResponse::BadRequest().body(format!("Error creating user {}", e));
    }
  };
}

pub async fn login(body: web::Json<NewUser>) -> impl Responder {
  use crate::schema::users::dsl::users;

  let connection = &mut connection();

  let input_user = body.into_inner();

  let user: User = match users.filter(username.eq(input_user.username)).select(User::as_select()).first(connection).optional() {
    Ok(result) => {
      match result {
        Some(user) => user,
        None => {
          return HttpResponse::BadRequest().body("Wrong username");
        }
      }
    }
    Err(_e) => {
      return HttpResponse::BadRequest().body("Wrong username");
    }
  };

  match User::verify_password(&user, &input_user.password) {
    Ok(result) => {
      if result {
        return HttpResponse::Ok().body("Login");
      } else {
        return HttpResponse::BadRequest().body("Wrong password");
      }
    }
    Err(_e) => {
      return HttpResponse::BadRequest().body("Cannot verify password");
    }
  };
}