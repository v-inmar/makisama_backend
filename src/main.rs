use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::Logger, web};
use dotenvy::dotenv;

use std::env;

mod handlers;
mod models;
mod repos;
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
    let db_host =
        env::var("DATABASE_HOST").expect("DATABASE_HOST must be set as environment variable");
    let db_port =
        env::var("DATABASE_PORT").expect("DATABASE_PORT must be set as environment variable");
    let db_schema =
        env::var("DATABASE_SCHEMA").expect("DATABASE_SCHEMA must be set as environment variable");
    let db_username = env::var("DATABASE_USERNAME")
        .expect("DATABASE_USERNAME must be set as environment variable");
    let db_password = env::var("DATABASE_PASSWORD")
        .expect("DATABASE_PASSWORD must be set as environment variable");
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_username, db_password, db_host, db_port, db_schema
    );

    let dbpool = utils::db::DatabasePool::new(&db_url)
        .await
        .expect("Failed to create database connection pool");

    match dbpool.ping().await {
        Ok(_val) => log::info!(
            "Connected to database {} on {}:{} with user {}",
            db_schema,
            db_host,
            db_port,
            db_username
        ),
        Err(e) => log::error!(
            "Error while connecting to database {} on {}:{}. {}",
            db_schema,
            db_host,
            db_port,
            e
        ),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(dbpool.pool.clone()))
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
