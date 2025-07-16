use actix_web::{HttpRequest, HttpResponse, Responder, get, web};

#[get("/{id}")]
pub async fn get_user(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    println!("user id: {}", user_id);
    if let Some(cookie) = req.cookie("refresh_token") {
        // Cookie found, return its value
        println!("Cookie value: {}", cookie.value());
    } else {
        // Cookie not found
        println!("No cookie found");
    }

    HttpResponse::Ok().body("user hit")
}
