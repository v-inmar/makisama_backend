use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(req: HttpRequest, form: web::Form<LoginForm>) -> impl Responder {
    let email = &form.email;
    let password = &form.password;

    println!("email: {}", email);
    println!("password: {}", password);

    HttpResponse::Ok().body("Login endpoint hit")
}
