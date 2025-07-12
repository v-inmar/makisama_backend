use actix_web::{HttpResponse, Responder, post};

#[post("/register")]
pub async fn register() -> impl Responder {
    HttpResponse::Created().body("Register endpoint hit")
}
