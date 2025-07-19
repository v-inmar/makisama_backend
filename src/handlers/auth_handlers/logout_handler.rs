use actix_web::{HttpRequest, HttpResponse, Responder, http::StatusCode};

use crate::utils::json_response_utils::JsonGeneralResponse;
use crate::utils::jwt_utils::decode_refresh_token;

pub async fn logout(req: HttpRequest) -> impl Responder {
    // Since this is a protected route, valid access token is necessary
    // already protected by middleware

    // Get cookie value
    let cookie = match req.cookie("refresh_token") {
        None => {
            return JsonGeneralResponse::make_response(
                &req,
                &StatusCode::UNAUTHORIZED,
                "Refresh token missing or invalid. Please log in again.",
            );
        }
        Some(cookie) => cookie,
    };

    // decode refresh token (the cookie value)
    // grab the auth id
    let token_data = match decode_refresh_token(cookie.value()) {
        Err(e) => {
            log::error!("{}", e);
            if e.to_string().to_lowercase() == "expiredsignature" {
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::OK,
                    "Logout successful",
                );
            } else {
                return JsonGeneralResponse::make_response(
                    &req,
                    &StatusCode::OK,
                    "Logout successful",
                );
            }
        }
        Ok(token_data) => token_data,
    };

    println!("{:?}", token_data);
    println!("{:?}", token_data.claims);

    // request ext should have the user's auth id
    // compare auth id from access token and refresh token are the same
    // this is to make sure refresh token is owned by the access token user (vice versa)

    // if let Some(cookie) = req.cookie("refresh_token") {
    //     // Cookie found, return its value
    //     println!("Cookie value: {}", cookie.value());
    // } else {
    //     // Cookie not found
    //     println!("No cookie found");
    // }

    HttpResponse::Ok().body("logout hit")
}
