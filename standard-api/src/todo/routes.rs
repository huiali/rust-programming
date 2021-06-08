use crate::todo::Todo;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

#[get("/todos")]
async fn find_all(db_pool: web::Data<MySqlPool>) -> impl Responder {
    // let t = req.0;
    // println!(
    //     "the handler is token:{:#?}",t
    // );
    let result = Todo::find_all(db_pool.get_ref()).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::BadRequest().body("Error trying to read all todos from database"),
    }
}

// #[get("/todo/{id}")]
// async fn find(id: web::Path<i32>, db_pool: web::Data<MySqlPool>) -> impl Responder {
//     let result = Todo::find_by_id(id.into_inner(), db_pool.get_ref()).await;
//     match result {
//         Ok(todo) => HttpResponse::Ok().json(todo),
//         _ => HttpResponse::BadRequest().body("Todo not found"),
//     }
// }

// #[post("/todo")]
// async fn create(todo: web::Json<TodoRequest>, db_pool: web::Data<MySqlPool>) -> impl Responder {
//     let result = Todo::create(todo.into_inner(), db_pool.get_ref()).await;
//     match result {
//         Ok(todo) => HttpResponse::Ok().json(todo),
//         _ => HttpResponse::BadRequest().body("Error trying to create new todo"),
//     }
// }

// #[put("/todo/{id}")]
// async fn update(
//     id: web::Path<i32>,
//     todo: web::Json<TodoRequest>,
//     db_pool: web::Data<MySqlPool>,
// ) -> impl Responder {
//     let result = Todo::update(id.into_inner(), todo.into_inner(), db_pool.get_ref()).await;
//     match result {
//         Ok(todo) => HttpResponse::Ok().json(todo),
//         _ => HttpResponse::BadRequest().body("Todo not found"),
//     }
// }

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    // cfg.service(find);
    // cfg.service(create);
    // cfg.service(update);
}
