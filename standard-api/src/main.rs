mod constants;
mod middleware;
mod response;
mod todo;
mod token;

#[macro_use]
extern crate lazy_static;

use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=Warn");
    env_logger::init();

    let database_url = "mysql://huiali@localhost:3306/todos"; // &env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .allowed_origin("http://127.0.0.1:3000")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(crate::middleware::auth_middleware::Authentication)
            .service(ping)
            .route("/hey", web::get().to(manual_hello))
            .configure(todo::init) // init todo routes
    })
    .keep_alive(3600)
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
