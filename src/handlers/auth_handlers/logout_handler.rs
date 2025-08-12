use actix_web::{HttpMessage, web};
use actix_web::{HttpRequest, Responder, http::StatusCode};
use chrono::{DateTime, Duration, Utc};
use sqlx::MySqlPool;

use crate::models::revoked_token_model::RevokedToken;
use crate::models::user_auth_identity_model::AuthIdentity;
use crate::models::user_model::User;
use crate::services::{auth_service, user_service};
use crate::utils::json_response_utils::JsonGeneralResponse;
use crate::utils::jwt_utils::decode_refresh_token;

pub async fn logout(req: HttpRequest, pool: web::Data<MySqlPool>) -> impl Responder {
    // Since this is a protected route, valid access token is necessary
    // already protected by middleware
    let sub = match req.extensions().get::<String>() {
        Some(value) => value.clone(),
        None => {
            // if for some other reason the middleware failed to do its job
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                "Must be authenticated",
            );
        }
    };

    // Get cookie value
    if let Some(cookie) = req.cookie("refresh_token") {
        // check if cookie (refresh token) had already been revoked and if it has
        // just return successful logout
        match RevokedToken::get_by_value(&pool, &cookie.value()).await {
            Err(e) => {
                log::error!("{}", e);
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error, try again later",
                );
            }
            Ok(Some(_)) => {
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::OK,
                    "Logout successful",
                );
            }
            Ok(None) => (),
        }

        // decode cookie (refresh token)
        match decode_refresh_token(cookie.value()) {
            Ok(token_data) => {
                // grab the access token sub (the user's auth id)

                // compare
                if sub.eq_ignore_ascii_case(&token_data.claims.sub) {
                    // grab the exp of the refresh token
                    if let Some(dt) = DateTime::from_timestamp(token_data.claims.exp, 0) {
                        // add 7 days to the exp time
                        let ttl = dt + Duration::days(7);

                        // revoked the refresh token
                        match auth_service::revoke_user_refresh_token(
                            &pool,
                            cookie.value(),
                            &ttl.naive_utc(),
                        )
                        .await
                        {
                            Err(e) => {
                                log::error!("{}", e); // let it continue to the end to change user's auth id
                            }
                            Ok(_) => {
                                return JsonGeneralResponse::make_response(
                                    &req,
                                    &StatusCode::OK,
                                    "Logout successful",
                                );
                            }
                        }
                    } // on else let it continue to change auth id of the user
                } // on else let it continue to change the uath id of the user
            }
            Err(e) => {
                log::error!("{}", e);
                if e.to_string().to_lowercase() == "expiredsignature" {
                    // still revoke this token to stop further reuse, just in case
                    // it has already expired so for datetime_ttl, use utc now + 7 days
                    let ttl = Utc::now() + Duration::days(7);
                    match auth_service::revoke_user_refresh_token(
                        &pool,
                        &cookie.value(),
                        &ttl.naive_utc(),
                    )
                    .await
                    {
                        Ok(_) => {
                            return JsonGeneralResponse::make_response(
                                &req,
                                &StatusCode::OK,
                                "Logout successful",
                            );
                        }
                        Err(e) => {
                            log::error!("{}", e);
                        }
                    }
                } // on else let it continue to change user's auth id
            }
        }
    } // on else let it continue to change user's auth id

    // Change auth id for security purposes since we can't revoked the refresh token
    // this will basically logout the user on all devices

    // 1. get the user that owns the auth_id_value
    match AuthIdentity::get_by_value(&pool, &sub).await {
        Err(e) => {
            log::error!("{}", e);
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Server error, try again later",
            );
        }
        Ok(None) => {
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                "Must be authenticated",
            );
        }
        Ok(Some(aio)) => match User::get_user_by_auth_identity_id(&pool, aio.id).await {
            Err(e) => {
                log::error!("{}", e);
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error, try again later",
                );
            }
            Ok(None) => {
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::UNAUTHORIZED,
                    "Must be authenticated",
                );
            }
            Ok(Some(user)) => match user_service::create_new_auth_id(&pool, &user).await {
                Ok(_) => {
                    return JsonGeneralResponse::make_response(
                        &req,
                        &StatusCode::OK,
                        "Logout successful",
                    );
                }
                Err(e) => {
                    log::error!("{}", e);
                    return JsonGeneralResponse::make_response(
                        &req,
                        &StatusCode::INTERNAL_SERVER_ERROR,
                        "Server error, try again later",
                    );
                }
            },
        },
    }
}
