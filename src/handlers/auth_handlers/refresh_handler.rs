use actix_web::{HttpMessage, HttpRequest, Responder, http::StatusCode, web};

use chrono::{DateTime, Duration};
use sqlx::MySqlPool;

use crate::{
    models::revoked_token_model::RevokedToken,
    services::auth_service,
    utils::{
        json_response_utils::{JsonGeneralResponse, JsonJwtResponse},
        jwt_utils::{self, decode_refresh_token},
    },
};

pub async fn refresh(req: HttpRequest, pool: web::Data<MySqlPool>) -> impl Responder {
    // At this point, req extension is sub (auth identity)
    let at_sub = match req.extensions().get::<String>() {
        Some(sub) => sub.clone(),
        None => {
            // if for some reason auth middleware failed to od its job
            // response with unauthorized
            // /refresh is STILL a protected route
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                "Access token is required",
            );
        }
    };

    // get the cookie (refresh token)
    if let Some(cookie) = req.cookie("refresh_token") {
        // Check if RT has already been revoked
        match RevokedToken::get_by_value(&pool, &cookie.value()).await {
            Err(e) => {
                log::error!("{}", e);
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error, try again later",
                );
            }
            Ok(Some(_rv)) => {
                // Token was already revoked
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::UNAUTHORIZED,
                    "Refresh token is no longer valid",
                );
            }
            Ok(None) => {
                // RT is still not revoked

                // decode RT
                let rt_token_data = match decode_refresh_token(&cookie.value()) {
                    Err(e) => {
                        if e.to_string().eq_ignore_ascii_case("expiredsignature")
                            || e.to_string().eq_ignore_ascii_case("invalidsignature")
                            || e.to_string().starts_with("base64 error")
                            || e.to_string().eq_ignore_ascii_case("invalidtoken")
                        {
                            let msg = format!("Refresh token {}", e);
                            return JsonGeneralResponse::make_response(
                                &req,
                                &StatusCode::UNAUTHORIZED,
                                &msg,
                            );
                        } else {
                            log::error!("{}", e);
                            return JsonGeneralResponse::make_response(
                                &req,
                                &StatusCode::INTERNAL_SERVER_ERROR,
                                "Server error, try again later.",
                            );
                        }
                    }
                    Ok(td) => td,
                };

                if at_sub.eq_ignore_ascii_case(&rt_token_data.claims.sub) {
                    // revoke RT
                    if let Some(dt) = DateTime::from_timestamp(rt_token_data.claims.exp, 0) {
                        let ttl = dt + Duration::days(7);
                        match auth_service::revoke_user_refresh_token(
                            &pool,
                            &cookie.value(),
                            &ttl.naive_utc(),
                        )
                        .await
                        {
                            Err(e) => {
                                log::error!("{}", e);
                                return JsonGeneralResponse::make_response(
                                    &req,
                                    &StatusCode::INTERNAL_SERVER_ERROR,
                                    "Server error, try again later",
                                );
                            }
                            Ok(_) => {
                                // generate AT
                                let new_at = match jwt_utils::generate_access_token(
                                    &rt_token_data.claims.sub,
                                ) {
                                    Err(e) => {
                                        log::error!("{}", e);
                                        return JsonGeneralResponse::make_response(
                                            &req,
                                            &StatusCode::INTERNAL_SERVER_ERROR,
                                            "Server error, try again later",
                                        );
                                    }
                                    Ok(access_token) => access_token,
                                };
                                // generate RT
                                let new_rt = match jwt_utils::generate_refresh_token(
                                    &rt_token_data.claims.sub,
                                ) {
                                    Err(e) => {
                                        log::error!("{}", e);
                                        return JsonGeneralResponse::make_response(
                                            &req,
                                            &StatusCode::INTERNAL_SERVER_ERROR,
                                            "Server error, try again later",
                                        );
                                    }
                                    Ok(refresh_token) => refresh_token,
                                };

                                return JsonJwtResponse::make_response(
                                    &req,
                                    &StatusCode::OK,
                                    &new_at,
                                    &new_rt,
                                );
                            }
                        }
                    } else {
                        return JsonGeneralResponse::make_response(
                            &req,
                            &StatusCode::INTERNAL_SERVER_ERROR,
                            "Server error, try again later",
                        );
                    }
                } else {
                    return JsonGeneralResponse::make_response(
                        &req,
                        &StatusCode::UNAUTHORIZED,
                        "Claim subs did not match. Please login",
                    );
                }

                // Compare AT sub and RT sub
                // if rt_token_data
                //     .claims
                //     .sub
                //     .eq_ignore_ascii_case(&sub.to_string())
                //     == false
                // {
                //     return JsonGeneralResponse::make_response(
                //         &req,
                //         &StatusCode::UNAUTHORIZED,
                //         "Claim sub did not match. Login required",
                //     );
                // }
            }
        }
    } else {
        JsonGeneralResponse::make_response(
            &req,
            &StatusCode::UNAUTHORIZED,
            "Refresh token is missing",
        )
    }

    // let sub = match req.extensions().get::<String>() {
    //     Some(value) => value.clone(),
    //     None => {
    //         // if for some other reason the middleware failed to do its job
    //         return JsonGeneralResponse::make_response(
    //             &req,
    //             &StatusCode::UNAUTHORIZED,
    //             "Must be authenticated",
    //         );
    //     }
    // };

    // let at_sub: String::new();
    // match jwt_utils::decode_access_token(&toke_or_sub) {
    //     Ok(token_data) => {
    //         at_sub =
    //     }
    // }

    // JsonGeneralResponse::make_response(&req, &StatusCode::OK, "Placeholder")
}
