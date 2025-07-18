use actix_web::{
    App, HttpServer,
    error::InternalError,
    http::StatusCode,
    middleware::Logger,
    web::{self},
};
use dotenvy::dotenv;

use std::env;

use crate::utils::json_response_utils::JsonGeneralResponse;

mod handlers;
mod middlewares;
mod models;
mod repositories;
mod services;
mod utils;

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
                let resp = JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::BAD_REQUEST,
                    &err.to_string().clone(),
                );
                InternalError::from_response(err, resp).into()
            }))
            .app_data(web::JsonConfig::default().error_handler(|err, req| {
                let resp = JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::BAD_REQUEST,
                    &err.to_string().clone(),
                );
                InternalError::from_response(err, resp).into()
            }))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(handlers::auth_handlers::login_handler::login)
                            .service(handlers::auth_handlers::register_handler::register)
                            .service(
                                // protected /auth endpoints
                                web::scope("")
                                    .wrap(middlewares::jwt_auth_middleware::AuthRequired {})
                                    .service(handlers::auth_handlers::logout_handler::logout),
                            ),
                    )
                    .service(
                        web::scope("/users")
                            .wrap(middlewares::jwt_auth_middleware::AuthRequired {})
                            .service(handlers::user_handlers::user_handler::get_user),
                    ),
            )
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
