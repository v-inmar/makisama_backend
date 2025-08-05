use actix_web::{
    App, HttpRequest, HttpServer,
    dev::{Service, ServiceResponse},
    error::InternalError,
    http::StatusCode,
    middleware::Logger,
    web::{self},
};
use dotenvy::dotenv;
use futures_util::FutureExt;

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
            .service(
                web::scope("/api")
                    // simple middleware for changing 405 response into the unified json response
                    // This only check for response status 405, anything else, it will not change the response
                    .wrap_fn(|req, srv| {
                        srv.call(req).map(|res| {
                            match res {
                                Err(ref e) => {
                                    log::error!("{}", e);
                                }
                                Ok(ref sr) => {
                                    if sr.status().as_u16() == 405 {
                                        let new_resp = JsonGeneralResponse::make_response(
                                            &sr.request(),
                                            &StatusCode::METHOD_NOT_ALLOWED,
                                            "Method not allowed for the requested endpoint",
                                        );

                                        return Ok(ServiceResponse::new(
                                            sr.request().clone(),
                                            new_resp,
                                        ));
                                    }
                                }
                            }
                            res
                        })
                    })
                    // any missing json key/value pairs that are expected
                    .app_data(web::JsonConfig::default().error_handler(|err, req| {
                        println!("1");
                        let resp = JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::BAD_REQUEST,
                            &err.to_string().clone(),
                        );
                        InternalError::from_response(err, resp).into()
                    }))
                    // any missing form key/value pairs that are expected
                    .app_data(web::FormConfig::default().error_handler(|err, req| {
                        let resp = JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::BAD_REQUEST,
                            &err.to_string().clone(),
                        );
                        InternalError::from_response(err, resp).into()
                    }))
                    .app_data(web::QueryConfig::default().error_handler(|err, req| {
                        let resp = JsonGeneralResponse::make_response(&req, &StatusCode::BAD_REQUEST, &err.to_string().clone());
                        InternalError::from_response(err, resp).into()
                    }))
                    .service(
                        web::scope("/auth")
                            .service(web::resource("/login").route(
                                web::post().to(handlers::auth_handlers::login_handler::login),
                            ))
                            .service(web::resource("/register").route(
                                web::post().to(handlers::auth_handlers::register_handler::register),
                            ))
                            .service(
                                web::scope("")
                                    .wrap(middlewares::jwt_auth_middleware::AuthRequired {})
                                    .service(
                                        web::resource("/refresh").route(
                                            web::post().to(
                                                handlers::auth_handlers::refresh_handler::refresh,
                                            ),
                                        ),
                                    )
                                    .service(
                                        web::resource("/logout").route(
                                            web::post().to(
                                                handlers::auth_handlers::logout_handler::logout,
                                            ),
                                        ),
                                    ),
                            ),
                    )
                    .service(
                        web::scope("/boards")
                            .wrap(middlewares::jwt_auth_middleware::AuthRequired {})
                            .service(
                                web::resource("")
                                    .route(
                                        web::post().to(
                                            handlers::board_handlers::add_board_handler::add_board,
                                        ),
                                    )
                                    .route(web::get().to(
                                        handlers::board_handlers::get_boards_handler::get_boards,
                                    )),
                            )
                            .service(
                                web::resource("/{id}")
                                    .name("get_board")
                                    .route(web::get().to(
                                        handlers::board_handlers::get_board_handler::get_board,
                                    ))
                                    .route(web::delete().to(handlers::board_handlers::delete_board_handler::delete_board)),
                            ),
                    ),
            )
            .default_service(web::route().to(|req: HttpRequest| async move {
                JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::NOT_FOUND,
                    "No matching endpoint",
                )
            }))
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
