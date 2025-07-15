use actix_web::{
    App, HttpResponse, HttpServer, Responder, error::InternalError, get, middleware::Logger, web,
};
use dotenvy::dotenv;

use std::env;

mod handlers;
mod models;
mod repositories;
mod services;
mod utils;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("API running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // load .env file
    dotenv().ok();

    // set and test database
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let dbpool = utils::db_utils::DatabasePool::new(&db_url)
        .await
        .expect("Failed to create database connection pool");

    match dbpool.ping().await {
        Ok(_val) => log::info!("Connected to database"),
        Err(e) => log::error!("Error while connecting to database. {}", e),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(dbpool.pool.clone()))
            .app_data(web::FormConfig::default().error_handler(|err, req| {
                InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().body("bad request form"),
                )
                .into()
            }))
            .app_data(web::JsonConfig::default().error_handler(|err, req| {
                InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().body("bad request json"),
                )
                .into()
            }))
            .service(
                web::scope("/api").service(health).service(
                    web::scope("/auth")
                        .service(handlers::auth_handlers::login_handler::login)
                        .service(handlers::auth_handlers::register_handler::register),
                ),
            )
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
