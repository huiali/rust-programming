mod todo;
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> Result<()> {
    let database_url = "mysql://root:huiali123@192.168.164.131:3306/todos"; // &env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .configure(todo::init) // init todo routes
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
