use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::http::StatusCode;
use actix_web::web;

use sqlx::MySqlPool;

use serde::Deserialize;
use serde::Serialize;

use crate::models::user_models::user_authid_model::UserAuthidModel;
use crate::models::user_models::user_email_model::UserEmailModel;
use crate::models::user_models::user_model::UserModel;
use crate::utils::bcrypt_utils::is_matched;
use crate::utils::jwt_utils::generate_access_token;
use crate::utils::jwt_utils::generate_refresh_token;
use crate::utils::response_utils::ResponseMaker;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestData {
    pub email: String,
    pub password: String,
}


pub struct Authentication {}

impl Authentication {
    pub async fn login(
        req: HttpRequest,
        pool: web::Data<MySqlPool>,
        data: web::Form<LoginRequestData>,
    ) -> impl Responder {
        // check email
        let user_email_obj: UserEmailModel =
            match UserEmailModel::get_by_value(&pool, &data.email).await {
                Err(e) => {
                    log::error!("{}", e);
                    return ResponseMaker::general_response(
                        &req,
                        &StatusCode::INTERNAL_SERVER_ERROR,
                        "Server Error. Try again later",
                    );
                }
                Ok(None) => {
                    return ResponseMaker::general_response(
                        &req,
                        &StatusCode::UNAUTHORIZED,
                        "Invalid email and/or password",
                    );
                }
                Ok(Some(e)) => e,
            };

        // get the user
        let user_obj: UserModel = match UserModel::get_by_email_id(&pool, user_email_obj.id).await {
            Err(e) => {
                log::error!("{}", e);
                return ResponseMaker::general_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error. Try again later",
                );
            }
            Ok(None) => {
                return ResponseMaker::general_response(
                    &req,
                    &StatusCode::UNAUTHORIZED,
                    "Invalid email and/or password",
                );
            }
            Ok(Some(u)) => u,
        };

        // check password
        match is_matched(&data.password, &user_obj.password) {
            Err(e) => {
                log::error!("{}", e);
                return ResponseMaker::general_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error. Try again later",
                );
            }
            Ok(m) => {
                if !m {
                    return ResponseMaker::general_response(
                        &req,
                        &StatusCode::UNAUTHORIZED,
                        "Invalid email and/or password",
                    );
                } else {
                    // password matched
                    // generate tokens

                    // get user authid
                    match UserAuthidModel::get_by_id(&pool, user_obj.authid_id).await {
                        Err(e) => {
                            log::error!("{}", e);
                            return ResponseMaker::general_response(
                                &req,
                                &StatusCode::INTERNAL_SERVER_ERROR,
                                "Server Error. Try again later",
                            );
                        }
                        Ok(None) => {
                            log::error!("Error! A user {} doesn't have authid", user_obj.id);
                            return ResponseMaker::general_response(
                                &req,
                                &StatusCode::INTERNAL_SERVER_ERROR,
                                "Server Error. Try again later",
                            );
                        }
                        Ok(Some(obj)) => {
                            // create access and refresh tokens
                            let access_token = match generate_access_token(&obj.value) {
                                Err(e) => {
                                    log::error!("{}", e);
                                    return ResponseMaker::general_response(
                                        &req,
                                        &StatusCode::INTERNAL_SERVER_ERROR,
                                        "Server Error. Try again later",
                                    );
                                }
                                Ok(at) => at,
                            };

                            let refresh_token = match generate_refresh_token(&obj.value) {
                                Err(e) => {
                                    log::error!("{}", e);
                                    return ResponseMaker::general_response(
                                        &req,
                                        &StatusCode::INTERNAL_SERVER_ERROR,
                                        "Server Error. Try again later",
                                    );
                                }
                                Ok(rt) => rt,
                            };

                            return ResponseMaker::jwt_response(
                                &req,
                                &StatusCode::OK,
                                &access_token,
                                &refresh_token,
                            );
                        }
                    }
                }
            }
        }
    }
}
