use actix_web::{HttpRequest, Responder, http::StatusCode};

use crate::utils::json_response_utils::JsonGeneralResponse;

pub async fn get_boards(req: HttpRequest) -> impl Responder {
    JsonGeneralResponse::make_response(&req, &StatusCode::OK, "OK")
}
