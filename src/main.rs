use actix_cors::Cors;
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

// use crate::utils::json_response_utils::JsonGeneralResponse;
use crate::utils::response_utils::ResponseMaker;

mod constants;
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
        // for testing - change for more secure options in prod
        let cors = Cors::default()
            .allowed_origin("*") // for testing only
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(dbpool.pool.clone()))
            .service(
                // initial scope /api
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
                                        let new_resp = ResponseMaker::general_response(
                                            &sr.request(),
                                            &StatusCode::METHOD_NOT_ALLOWED,
                                            "Method not allowed for the requested endpoint",
                                        );
                                        // let new_resp = JsonGeneralResponse::make_response(
                                        //     &sr.request(),
                                        //     &StatusCode::METHOD_NOT_ALLOWED,
                                        //     "Method not allowed for the requested endpoint",
                                        // );

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
                        let resp = ResponseMaker::general_response(
                            &req,
                            &StatusCode::BAD_REQUEST,
                            &err.to_string().clone(),
                        );
                        // let resp = JsonGeneralResponse::make_response(
                        //     &req,
                        //     &StatusCode::BAD_REQUEST,
                        //     &err.to_string().clone(),
                        // );
                        InternalError::from_response(err, resp).into()
                    }))
                    // any missing form key/value pairs that are expected
                    .app_data(web::FormConfig::default().error_handler(|err, req| {
                        // let resp = JsonGeneralResponse::make_response(
                        //     &req,
                        //     &StatusCode::BAD_REQUEST,
                        //     &err.to_string().clone(),
                        // );
                        let resp = ResponseMaker::general_response(
                            &req,
                            &StatusCode::BAD_REQUEST,
                            &err.to_string().clone(),
                        );
                        InternalError::from_response(err, resp).into()
                    }))
                    // any errors associated with query params before it reached the handler
                    .app_data(web::QueryConfig::default().error_handler(|err, req| {
                        // let resp = JsonGeneralResponse::make_response(
                        //     &req,
                        //     &StatusCode::BAD_REQUEST,
                        //     &err.to_string().clone(),
                        // );
                        let resp = ResponseMaker::general_response(
                            &req,
                            &StatusCode::BAD_REQUEST,
                            &err.to_string().clone(),
                        );
                        InternalError::from_response(err, resp).into()
                    }))
                    // services associated with /api scope
                    .service(
                        // auth scope - /api/auth
                        web::scope("/auth")
                            // login service - /api/auth/login
                            .service(web::resource("/login").route(
                                web::post().to(handlers::authentication::Authentication::login),
                            ))
                            // register service - /api/auth/register
                            .service(web::resource("/register").route(
                                web::post().to(handlers::authentication::Authentication::register),
                            ))
                            .service(
                                // empty scope, still corresponds to /api/auth
                                web::scope("")
                                    // auth middleware - check that any endpoint after this scope must be authenticated
                                    // when calling a service
                                    .wrap(middlewares::jwt_auth_middleware::AuthRequired {})
                                    // refresh token service - /api/auth/refresh
                                    .service(
                                        web::resource("/refresh").route(
                                            web::post().to(
                                                handlers::authentication::Authentication::refresh,
                                            ),
                                        ),
                                    )
                                    // logout service - /api/auth/logout
                                    .service(
                                        web::resource("/logout").route(
                                            web::post().to(
                                                handlers::authentication::Authentication::logout,
                                            ),
                                        ),
                                    ),
                            ),
                    ), // services associated with /api scope
                       // .service(

                       //     // boards scope - /api/boards
                       //     web::scope("/boards")

                       //         // auth middleware for protecting endpoints
                       //         .wrap(middlewares::jwt_auth_middleware::AuthRequired {})

                       //         // corresponds to /api/boards - root of baords
                       //         .service(
                       //             web::resource("")
                       //                 .route(
                       //                     web::post().to(
                       //                         handlers::board_handlers::add_board_handler::add_board,
                       //                     ),
                       //                 )
                       //                 .route(web::get().to(
                       //                     handlers::board_handlers::get_boards_handler::get_boards,
                       //                 )),
                       //         )

                       //         // service for single board - /api/boards/1234
                       //         .service(
                       //             web::resource("/{pid}")
                       //                 .name("get_board") // resource name so it can be used in url_for
                       //                 .route(web::get().to(
                       //                     handlers::board_handlers::get_board_handler::get_board,
                       //                 ))
                       //                 .route(web::delete().to(handlers::board_handlers::delete_board_handler::delete_board)),
                       //         ),
                       // ),
            )
            // default service - not existent endpoints
            .default_service(web::route().to(|req: HttpRequest| async move {
                ResponseMaker::general_response(
                    &req,
                    &StatusCode::NOT_FOUND,
                    "No matching endpoint",
                )
                // JsonGeneralResponse::make_response(
                //     &req,
                //     &StatusCode::NOT_FOUND,
                //     "No matching endpoint",
                // )
            }))
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
