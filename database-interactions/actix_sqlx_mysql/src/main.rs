use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "mysql://root:huiali123@192.168.164.131:3306/todos"; // &env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new().connect(database_url).await?;

    HttpServer::new(|| {
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    let sql = "select `id`,`key`,`description` AS description from todos";
    let recs = sqlx::query!("select `id`,`key`,`description` AS description from todos")
        .fetch_all(pool)
        .await?;

    let result = sqlx::query(sql).fetch_all(pool).await?;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(result)
    // HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
