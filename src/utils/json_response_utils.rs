use actix_web::{
    HttpRequest, HttpResponse, cookie::CookieBuilder, cookie::time::Duration, http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDetails {
    pub path: String,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusDetails {
    pub code: u16,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonErrorResponse {
    pub request_details: RequestDetails,
    pub status_details: StatusDetails,
    pub message: String,
}

impl JsonErrorResponse {
    pub fn make_response(req: &HttpRequest, code: &StatusCode, msg: &str) -> HttpResponse {
        let mut response_builder = HttpResponse::build(*code);
        response_builder.content_type("application/json");

        let status_details = StatusDetails {
            code: code.as_u16(),
            status: code
                .canonical_reason()
                .unwrap_or("Internal Server Error")
                .to_string(),
        };

        let request_details = RequestDetails {
            path: req.path().to_string(),
            method: req.method().to_string(),
        };

        response_builder.json(JsonErrorResponse {
            request_details: request_details,
            status_details: status_details,
            message: msg.to_string(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonJwtResponse {
    pub request_details: RequestDetails,
    pub status_details: StatusDetails,
    pub access_token: String,
}

impl JsonJwtResponse {
    pub fn make_response(
        req: &HttpRequest,
        code: &StatusCode,
        access_token: &str,
        refresh_token: &str,
    ) -> HttpResponse {
        let mut response_builder = HttpResponse::build(*code);
        response_builder.content_type("application/json");

        let refresh_cookie = CookieBuilder::new("refresh_token", refresh_token)
            .http_only(true)
            .secure(false) // TODO true on prod
            .same_site(actix_web::cookie::SameSite::Strict)
            .max_age(Duration::days(7))
            .path("/api")
            .finish();

        response_builder.cookie(refresh_cookie);

        let status_details = StatusDetails {
            code: code.as_u16(),
            status: code
                .canonical_reason()
                .unwrap_or("Internal Server Error")
                .to_string(),
        };

        let request_details = RequestDetails {
            path: req.path().to_string(),
            method: req.method().to_string(),
        };

        response_builder.json(JsonJwtResponse {
            request_details: request_details,
            status_details: status_details,
            access_token: access_token.to_string(),
        })
    }
}
