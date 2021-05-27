use crate::todo::Todo;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

#[get("/todos")]
async fn find_all(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Todo::find_all(pool.get_ref()).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::BadRequest().body("Error trying to read all todos from database"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    // cfg.service(find);
    // cfg.service(create);
    // cfg.service(update);
    // cfg.service(delete);
}
