use actix_web::{
    HttpRequest, HttpResponse, Responder, cookie::CookieBuilder, cookie::time::Duration, post, web,
};

use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::{
    models::{auth_identity_model::AuthIdentity, user_model::User},
    utils::{
        bcrypt_utils::is_matched,
        jwt_utils::{generate_access_token, generate_refresh_token},
    },
};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    form: web::Form<LoginForm>,
) -> impl Responder {
    let email = &form.email;
    let password = &form.password;

    let user = match User::get_user_by_email(&pool, email).await {
        Err(e) => {
            log::error!("Unable to get user by email. {}", e);
            return HttpResponse::InternalServerError().body("Server error, try again later");
        }
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid email/password"),
    };

    let hashed = &user.password;
    match is_matched(&password, hashed) {
        Err(e) => {
            log::error!("Unable to check password. {}", e);
            HttpResponse::InternalServerError().body("Server error, try again later")
        }
        Ok(matched) => {
            if !matched {
                HttpResponse::Unauthorized().body("Invalid email/password")
            } else {
                match AuthIdentity::get_by_id(&pool, user.auth_identity_id).await {
                    Err(e) => {
                        log::error!("Unable to get auth identity. {}", e);
                        HttpResponse::InternalServerError().body("Server error, try again later")
                    }
                    Ok(None) => {
                        log::error!("No auth identity for user.");
                        HttpResponse::InternalServerError().body("Server error, try again later")
                    }
                    Ok(Some(aio)) => {
                        let access_token = match generate_access_token(&aio.value) {
                            Ok(token) => token,
                            Err(e) => {
                                log::error!("Unable to generate access token user. {}", e);
                                return HttpResponse::InternalServerError()
                                    .body("Server error, try again later");
                            }
                        };

                        let refresh_token = match generate_refresh_token(&aio.value) {
                            Ok(token) => token,
                            Err(e) => {
                                log::error!("Unable to generate refresh token for user. {}", e);
                                return HttpResponse::InternalServerError()
                                    .body("Server error, try again later");
                            }
                        };

                        let refresh_cookie = CookieBuilder::new("refresh_token", refresh_token)
                            .http_only(true)
                            .secure(false) // true on prod
                            .same_site(actix_web::cookie::SameSite::Strict)
                            .max_age(Duration::days(7))
                            .finish();

                        HttpResponse::Ok().cookie(refresh_cookie).body(access_token)
                    }
                }
            }
        }
    }
}
