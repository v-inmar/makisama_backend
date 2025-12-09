use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::http::StatusCode;
use actix_web::web;

use chrono::DateTime;

use chrono::Duration;
use jsonwebtoken::TokenData;
use sqlx::MySqlPool;

use serde::Deserialize;
use serde::Serialize;

use validator::Validate;

use crate::constants;
use crate::models::revoked_token_models::revoked_token_model::RevokedTokenModel;
use crate::models::user_models::user_authid_model::UserAuthidModel;
use crate::models::user_models::user_email_model::UserEmailModel;
use crate::models::user_models::user_model::UserModel;
use crate::services::auth_service::AuthService;
use crate::services::user_service::UserService;
use crate::utils::bcrypt_utils::is_matched;
use crate::utils::custom_validation_utils::validate_email;
use crate::utils::custom_validation_utils::validate_name;

use crate::utils::jwt_utils::Claims;
use crate::utils::jwt_utils::decode_refresh_token;
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
        /*
           - Get email from form
               - check if its present in db
               - get user
           - Check user password with the password that came with the form
           - Create access token and refresh token
           - Access token goes to response body and refresh token goes into cookie
        */

        // check email
        let user_email_obj: UserEmailModel =
            match UserEmailModel::get_by_value(&pool, &data.email).await {
                Err(e) => {
                    log::error!("{}", e);
                    return ResponseMaker::respond_with_server_error(&req);
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
                return ResponseMaker::respond_with_server_error(&req);
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
                return ResponseMaker::respond_with_server_error(&req);
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
                            return ResponseMaker::respond_with_server_error(&req);
                        }
                        Ok(None) => {
                            log::error!("Error! A user {} doesn't have authid", user_obj.id);
                            return ResponseMaker::respond_with_server_error(&req);
                        }
                        Ok(Some(obj)) => {
                            // create access and refresh tokens
                            let access_token = match generate_access_token(&obj.value) {
                                Err(e) => {
                                    log::error!("{}", e);
                                    return ResponseMaker::respond_with_server_error(&req);
                                }
                                Ok(at) => at,
                            };

                            let refresh_token = match generate_refresh_token(&obj.value) {
                                Err(e) => {
                                    log::error!("{}", e);
                                    return ResponseMaker::respond_with_server_error(&req);
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
        /*
            - Validate the data
            - Check that form password and form repeat are the same
            - Check email is not yet in use
            - Create the user and its associated data
            - Create and respond with access token and refresh token - access token goes into body, refresh token goes to cookie
        */

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
                return ResponseMaker::respond_with_server_error(&req);
            }
            Ok(None) => {}
            Ok(Some(uem)) => {
                match UserModel::get_by_email_id(&pool, uem.id).await {
                    Err(e) => {
                        log::error!("{}", e);
                        return ResponseMaker::respond_with_server_error(&req);
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
                return ResponseMaker::respond_with_server_error(&req);
            }
            Ok(u) => u,
        };

        // get user authid and create tokens
        let user_authid_obj: UserAuthidModel =
            match UserAuthidModel::get_by_id(&pool, user_obj.authid_id).await {
                Err(e) => {
                    log::error!("{}", e);
                    return ResponseMaker::respond_with_server_error(&req);
                }
                Ok(None) => {
                    log::error!(
                        "Unable to get user auth identity id from newly created user. user id: {}",
                        user_obj.id
                    );
                    return ResponseMaker::respond_with_server_error(&req);
                }
                Ok(Some(uam)) => uam,
            };

        let access_token: String = match generate_access_token(&user_authid_obj.value) {
            Err(e) => {
                log::error!("{}", e);
                return ResponseMaker::respond_with_server_error(&req);
            }
            Ok(at) => at,
        };

        let refresh_token: String = match generate_refresh_token(&user_authid_obj.value) {
            Err(e) => {
                log::error!("{}", e);
                return ResponseMaker::respond_with_server_error(&req);
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
        /*
            - Get the authid from access token - auth middleware already takes care of decoding the access token
            - Get the refresh token from cookie
                - check that it is not in db yet
                - decode the refresh token to get TokenData<Claim>
            - Compare and make sure that access token authid == refresh token authid
                - create new access token and refresh token
            - Use jwt response to make response and send it back to client
        */

        // get access token
        // At this point, req extension is sub (authid)
        let access_token_authid_value: String = match req.extensions().get::<String>() {
            Some(sub) => sub.clone(),
            None => {
                return ResponseMaker::respond_with_server_error(&req);
            }
        };

        // get refresh token from cookie
        if let Some(cookie) = req.cookie("refresh_token") {
            // check if refresh token has already been revoked
            match RevokedTokenModel::get_by_value(&pool, &cookie.value()).await {
                Err(e) => {
                    log::error!("{}", e);
                    return ResponseMaker::respond_with_server_error(&req);
                }
                Ok(Some(_)) => {
                    return ResponseMaker::general_response(
                        &req,
                        &StatusCode::UNAUTHORIZED,
                        "Refresh token is invalid",
                    );
                }
                Ok(None) => {
                    // decode the refresh token
                    // cookie.value() is refresh token
                    let refresh_token_td: TokenData<Claims> =
                        match decode_refresh_token(&cookie.value()) {
                            Err(e) => {
                                let (err_msg, status) =
                                    if e.to_string().eq_ignore_ascii_case("expiredsignature") {
                                        (
                                            "Refresh token has an expired signature",
                                            &StatusCode::UNAUTHORIZED,
                                        )
                                    } else if e.to_string().eq_ignore_ascii_case("invalidsignature")
                                    {
                                        (
                                            "Refresh token has an invalid signature",
                                            &StatusCode::UNAUTHORIZED,
                                        )
                                    } else if e.to_string().starts_with("Base64") {
                                        (
                                            "Refresh token has Base64 encoding issue",
                                            &StatusCode::UNAUTHORIZED,
                                        )
                                    } else if e.to_string().eq_ignore_ascii_case("invalidtoken") {
                                        ("Refresh token is invalid", &StatusCode::UNAUTHORIZED)
                                    } else {
                                        (
                                            constants::INTERNAL_SERVER_ERROR_MSG,
                                            &StatusCode::INTERNAL_SERVER_ERROR,
                                        )
                                    };

                                return ResponseMaker::general_response(&req, &status, err_msg);
                            }
                            Ok(td) => td,
                        };

                    if !access_token_authid_value.eq_ignore_ascii_case(&refresh_token_td.claims.sub)
                    {
                        return ResponseMaker::general_response(
                            &req,
                            &StatusCode::UNAUTHORIZED,
                            "Claim subs did not match. Please login",
                        );
                    } else {
                        if let Some(dt) = DateTime::from_timestamp(refresh_token_td.claims.exp, 0) {
                            let ttl = dt + Duration::days(7);

                            match AuthService::create_revoked(
                                &pool,
                                &cookie.value(),
                                &ttl.naive_utc(),
                            )
                            .await
                            {
                                Err(e) => {
                                    log::error!("{}", e);
                                    return ResponseMaker::respond_with_server_error(&req);
                                }
                                Ok(_) => {
                                    // generate access token
                                    let new_access_token =
                                        match generate_access_token(&refresh_token_td.claims.sub) {
                                            Err(e) => {
                                                log::error!("{}", e);
                                                return ResponseMaker::respond_with_server_error(
                                                    &req,
                                                );
                                            }
                                            Ok(at) => at,
                                        };

                                    // generate refresh token
                                    let new_refresh_token = match generate_refresh_token(
                                        &refresh_token_td.claims.sub,
                                    ) {
                                        Err(e) => {
                                            log::error!("{}", e);
                                            return ResponseMaker::respond_with_server_error(&req);
                                        }
                                        Ok(rt) => rt,
                                    };

                                    // respond with new access token and new cookie with refresh token
                                    return ResponseMaker::jwt_response(
                                        &req,
                                        &StatusCode::OK,
                                        &new_access_token,
                                        &new_refresh_token,
                                    );
                                }
                            }
                        } else {
                            return ResponseMaker::respond_with_server_error(&req);
                        }
                    }
                }
            }
        } else {
            return ResponseMaker::general_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                "Refresh token cookie is missing",
            );
        }
    }
}
