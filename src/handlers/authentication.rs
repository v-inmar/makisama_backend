use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::http::StatusCode;
use actix_web::web;

use sqlx::MySqlPool;

use serde::Deserialize;
use serde::Serialize;

use validator::Validate;

use crate::constants;
use crate::models::user_models::user_authid_model::UserAuthidModel;
use crate::models::user_models::user_email_model::UserEmailModel;
use crate::models::user_models::user_model::UserModel;
use crate::services::user_service::UserService;
use crate::utils::bcrypt_utils::is_matched;
use crate::utils::custom_validation_utils::validate_email;
use crate::utils::custom_validation_utils::validate_name;

use crate::utils::jwt_utils::generate_access_token;
use crate::utils::jwt_utils::generate_refresh_token;
use crate::utils::response_utils::ResponseMaker;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestData {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterRequestData {
    #[validate(custom(function = "validate_name"))]
    pub firstname: String,

    #[validate(custom(function = "validate_name"))]
    pub lastname: String,

    #[validate(custom(function = "validate_email"))]
    pub email: String,

    #[validate(length(min = 8, max = 255))]
    pub password: String,

    #[validate(length(min = 8, max = 255))]
    pub repeat: String,
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
                        constants::INTERNAL_SERVER_ERROR_MSG,
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
                    constants::INTERNAL_SERVER_ERROR_MSG,
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
                    constants::INTERNAL_SERVER_ERROR_MSG,
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
                                constants::INTERNAL_SERVER_ERROR_MSG,
                            );
                        }
                        Ok(None) => {
                            log::error!("Error! A user {} doesn't have authid", user_obj.id);
                            return ResponseMaker::general_response(
                                &req,
                                &StatusCode::INTERNAL_SERVER_ERROR,
                                constants::INTERNAL_SERVER_ERROR_MSG,
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
                                        constants::INTERNAL_SERVER_ERROR_MSG,
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
                                        constants::INTERNAL_SERVER_ERROR_MSG,
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

    pub async fn register(
        req: HttpRequest,
        pool: web::Data<MySqlPool>,
        data: web::Json<RegisterRequestData>,
    ) -> impl Responder {
        // validate the incoming data
        match data.validate() {
            Ok(_) => {}
            Err(e) => {
                return ResponseMaker::general_response(
                    &req,
                    &StatusCode::BAD_REQUEST,
                    e.to_string(),
                );
            }
        }

        // check password == repeat
        if data.password != data.repeat {
            return ResponseMaker::general_response(
                &req,
                &StatusCode::BAD_REQUEST,
                "Password and Repeat did not match",
            );
        }

        // check email if in use
        match UserEmailModel::get_by_value(&pool, &data.email).await {
            Err(e) => {
                log::error!("{}", e);
                return ResponseMaker::general_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    constants::INTERNAL_SERVER_ERROR_MSG,
                );
            }
            Ok(None) => {}
            Ok(Some(uem)) => {
                match UserModel::get_by_email_id(&pool, uem.id).await {
                    Err(e) => {
                        log::error!("{}", e);
                        return ResponseMaker::general_response(
                            &req,
                            &StatusCode::INTERNAL_SERVER_ERROR,
                            constants::INTERNAL_SERVER_ERROR_MSG,
                        );
                    }
                    Ok(None) => {}
                    Ok(Some(_)) => {
                        // already in use
                        return ResponseMaker::general_response(
                            &req,
                            &StatusCode::CONFLICT,
                            "Email address is already in use",
                        );
                    }
                }
            }
        }

        // create user
        let user_obj: UserModel = match UserService::create_user(&pool, &data).await {
            Err(e) => {
                log::error!("{}", e);
                return ResponseMaker::general_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    constants::INTERNAL_SERVER_ERROR_MSG,
                );
            }
            Ok(u) => u,
        };

        // get user authid and create tokens
        let user_authid_obj: UserAuthidModel =
            match UserAuthidModel::get_by_id(&pool, user_obj.authid_id).await {
                Err(e) => {
                    log::error!("{}", e);
                    return ResponseMaker::general_response(
                        &req,
                        &StatusCode::INTERNAL_SERVER_ERROR,
                        constants::INTERNAL_SERVER_ERROR_MSG,
                    );
                }
                Ok(None) => {
                    log::error!(
                        "Unable to get user auth identity id from newly created user. user id: {}",
                        user_obj.id
                    );
                    return ResponseMaker::general_response(
                        &req,
                        &StatusCode::INTERNAL_SERVER_ERROR,
                        constants::INTERNAL_SERVER_ERROR_MSG,
                    );
                }
                Ok(Some(uam)) => uam,
            };

        let access_token: String = match generate_access_token(&user_authid_obj.value) {
            Err(e) => {
                log::error!("{}", e);
                return ResponseMaker::general_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    constants::INTERNAL_SERVER_ERROR_MSG,
                );
            }
            Ok(at) => at,
        };

        let refresh_token: String = match generate_refresh_token(&user_authid_obj.value) {
            Err(e) => {
                log::error!("{}", e);
                return ResponseMaker::general_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    constants::INTERNAL_SERVER_ERROR_MSG,
                );
            }
            Ok(rt) => rt,
        };

        return ResponseMaker::jwt_response(
            &req,
            &StatusCode::CREATED,
            &access_token,
            &refresh_token,
        );
    }

    pub async fn logout(req: HttpRequest, pool: web::Data<MySqlPool>) -> impl Responder {
        return ResponseMaker::general_response(&req, &StatusCode::OK, "Ok logout");
    }

    pub async fn refresh(req: HttpRequest, pool: web::Data<MySqlPool>) -> impl Responder {
        return ResponseMaker::general_response(&req, &StatusCode::OK, "Ok refresh");
    }
}
