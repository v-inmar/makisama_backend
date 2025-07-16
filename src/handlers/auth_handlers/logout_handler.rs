use actix_web::{HttpRequest, HttpResponse, Responder, post};

#[post("/logout")]
pub async fn logout(req: HttpRequest) -> impl Responder {
    if let Some(cookie) = req.cookie("refresh_token") {
        // Cookie found, return its value
        println!("Cookie value: {}", cookie.value());
    } else {
        // Cookie not found
        println!("No cookie found");
    }

    HttpResponse::Ok().body("logout hit")
}
