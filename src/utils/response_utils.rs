use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::cookie::CookieBuilder;
use actix_web::cookie::time::Duration;
use actix_web::http::StatusCode;

use serde::Deserialize;
use serde::Serialize;

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
pub struct ResponseDetails<T> {
    request_details: RequestDetails,
    status_details: StatusDetails,
    payload: T,
}

pub struct ResponseMaker {}

impl ResponseMaker {
    pub fn general_response<T: Serialize>(
        req: &HttpRequest,
        code: &StatusCode,
        payload: T,
    ) -> HttpResponse {
        let mut resp_builder = HttpResponse::build(*code);
        resp_builder.content_type("application/json");
        return resp_builder.json(ResponseDetails {
            request_details: _get_request_details(&req),
            status_details: _get_status_details(&code),
            payload,
        });
    }

    pub fn jwt_response(
        req: &HttpRequest,
        code: &StatusCode,
        access_token: &str,
        refresh_token: &str,
    ) -> HttpResponse {
        let mut resp_builder = HttpResponse::build(*code);
        resp_builder.content_type("application/json");

        let refresh_cookie = CookieBuilder::new("refresh_token", refresh_token)
            .http_only(true)
            .secure(false) // TODO true on prod
            // TODO this on prod    .same_site(actix_web::cookie::SameSite::Strict)
            .same_site(actix_web::cookie::SameSite::Lax)
            .max_age(Duration::days(7))
            .path("/api/auth")
            .finish();

        resp_builder.cookie(refresh_cookie);

        return resp_builder.json(ResponseDetails {
            request_details: _get_request_details(&req),
            status_details: _get_status_details(&code),
            payload: access_token,
        });
    }
}

fn _get_request_details(req: &HttpRequest) -> RequestDetails {
    RequestDetails {
        path: req.path().to_string(),
        method: req.method().to_string(),
    }
}

fn _get_status_details(code: &StatusCode) -> StatusDetails {
    StatusDetails {
        code: code.as_u16(),
        status: code
            .canonical_reason()
            .unwrap_or("Status Undefined")
            .to_string(),
    }
}
