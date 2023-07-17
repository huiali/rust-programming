mod constants;
mod query;
mod response;
mod sample;
use actix_cors::Cors;
use actix_web::{error, get, http::KeepAlive, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{Ok, Result};
use log::info;
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let mongo_client = self::init_mongo().await?;
    info!("Starting...output::huiali");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .send_wildcard()
                    .allowed_origin_fn(|_, _req_head| true)
                    .allowed_methods(vec!["OPTIONS", "GET", "POST", "PUT", "DELETE"])
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(actix_web::middleware::Logger::default())
            .app_data(mongo_client.clone())
            .app_data(
                web::JsonConfig::default()
                    .limit(200 * 1024 * 1024) // Limit request payload size
                    .error_handler(|err, _| {
                        let resp_err = err.to_string();
                        error::InternalError::from_response(
                            err,
                            HttpResponse::BadRequest().body(resp_err),
                        )
                        .into()
                    }),
            )
            .service(ping)
            .configure(sample::init)
    })
    .bind("0.0.0.0:8000")?
    .workers(3)
    .keep_alive(KeepAlive::Os)
    .run()
    .await?;
    Ok(())
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}

async fn init_mongo() -> Result<web::Data<Mutex<Client>>> {
    let mongodb_url = env::var("MONGODB_SERVER_URL").expect("MONGODB_SERVER_URL not found.");
    let client_options = ClientOptions::parse(mongodb_url).await?;
    Ok(web::Data::new(Mutex::new(Client::with_options(
        client_options,
    )?)))
}
