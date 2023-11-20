pub mod schema;
pub mod models;
pub mod database;
pub mod api;
pub mod routes;
pub mod webstruct;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use routes::users::users_config;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                .route("", web::get().to(hello))
            )
            .service(
                web::scope("/api")
                .configure(users_config)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
