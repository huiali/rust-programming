#![warn(missing_docs, missing_debug_implementations)]
use crate::constants::EMPTY;
use crate::query::IdQuery;
use crate::query::RIdQuery;
use crate::response::ResponseBody;
use crate::sample::Sample;
use actix_web::delete;
use actix_web::put;
use actix_web::{get, post, web, HttpResponse, Responder};
use bson::oid::ObjectId;
use mongodb::Client;
use std::sync::Mutex;

#[get("/samples")]
async fn find_all(data: web::Data<Mutex<Client>>, req: web::Query<RIdQuery>) -> impl Responder {
    let result = Sample::find_all(data, &req.0.rid, &String::from("001")).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(ResponseBody::new(EMPTY, res, 1)),
        err => HttpResponse::BadRequest().json(ResponseBody::new(
            format!("{:?}", err).as_str(),
            EMPTY,
            0,
        )),
    }
}

#[post("/sample")]
async fn create(data: web::Data<Mutex<Client>>, req: web::Json<Sample>) -> impl Responder {
    let result = Sample::create(data, req.0, String::from("001")).await;
    match result {
        Ok(op) => match op {
            Some(res) => HttpResponse::Created().json(ResponseBody::new(EMPTY, res, 1)),
            None => HttpResponse::Ok().json(ResponseBody::new(
                "新增失败",
                EMPTY,
                0,
            )),
        },
        err => {
            return HttpResponse::BadRequest().json(ResponseBody::new(
                format!("{:?}", err).as_str(),
                EMPTY,
                0,
            ))
        }
    }
}

#[put("/sample")]
async fn update(
    data: web::Data<Mutex<Client>>,
    query: web::Query<IdQuery>,
    req: web::Json<Sample>,
) -> impl Responder {
    let result = Sample::update(data.clone(), req.0, &query.0.id, &String::from("001")).await;
    match result {
        Ok(op) => match op {
            Some(res) => return HttpResponse::Ok().json(ResponseBody::new(EMPTY, res, 1)),
            None => {
                return HttpResponse::Ok().json(ResponseBody::new(
                    "修改失败",
                    EMPTY,
                    0,
                ))
            }
        },
        err => {
            return HttpResponse::BadRequest().json(ResponseBody::new(
                format!("{:?}", err).as_str(),
                EMPTY,
                0,
            ))
        }
    }
}

#[delete("/sample")]
async fn delete(data: web::Data<Mutex<Client>>, query: web::Query<IdQuery>) -> impl Responder {
    let oid = ObjectId::parse_str(&query.0.id)
        .map_err(|e| {
            return HttpResponse::NotFound().body(format!("{}", e));
        })
        .unwrap();
    let ids_to_delete = vec![oid];
    let result = Sample::delete(data, ids_to_delete).await;
    match result {
        Ok(op) => match op {
            Some(res) => return HttpResponse::Ok().json(ResponseBody::new(EMPTY, res, 1)),
            None => {
                return HttpResponse::Ok().json(ResponseBody::new(
                    "删除失败",
                    EMPTY,
                    0,
                ))
            }
        },
        err => {
            return HttpResponse::BadRequest().json(ResponseBody::new(
                format!("{:?}", err).as_str(),
                EMPTY,
                0,
            ))
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
