use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub key: String,
    pub description: Option<String>,
}

impl Responder for Todo {
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        todo!()
    }
}

impl Todo {
    pub async fn find_all(pool: &MySqlPool) -> Result<Vec<Todo>> {
        let todos = vec![];
        let recs = sqlx::query("SELECT id, key, description FROM todos ORDER BY id")
            .fetch_all(pool)
            .await?;
        for rec in recs {
            // todos.push(Todo {
            //     id: 1,
            //     key: String::from("1"),
            //     description: String::from("123"),
            // });
        }
        Ok(todos)
    }
}
