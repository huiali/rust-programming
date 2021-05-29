// mod middleware;
mod todo;
mod constants;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> Result<()> {
    let database_url = "mysql://dbn_managers:PePRBQRKM9@AHkBx1Vh@10.221.253.27:5000/todos"; // &env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .configure(todo::init) // init todo routes
    })
    .keep_alive(3600)
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
