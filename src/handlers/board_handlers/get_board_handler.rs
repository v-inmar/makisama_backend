use actix_web::{HttpRequest, Responder, http::StatusCode, web};

use crate::utils::json_response_utils::JsonGeneralResponse;

pub async fn get_board(req: HttpRequest, id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    JsonGeneralResponse::make_response(&req, &StatusCode::OK, format!("OK {}", id).as_str())
}
