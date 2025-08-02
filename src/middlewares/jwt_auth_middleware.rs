use std::future::{Ready, ready};
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::HttpMessage; // for extension_mut()
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};

use futures_util::future::LocalBoxFuture;

use crate::utils::header_utils::RequestHeader;
use crate::utils::json_response_utils::JsonGeneralResponse;
use crate::utils::jwt_utils::{decode_access_token, decode_access_token_no_validation_exp};

pub struct AuthRequired {}

impl<S> Transform<S, ServiceRequest> for AuthRequired
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthRequiredMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthRequiredMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthRequiredMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthRequiredMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        // get the authorization header value
        let access_token = match req.get_header_value("authorization") {
            Err(e) => {
                log::error!("{}", e);

                let resp = JsonGeneralResponse::make_response(
                    &req.request(),
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error, try again later",
                );
                return Box::pin(async move { Ok(req.into_response(resp.map_into_boxed_body())) });
            }
            Ok(None) => {
                let resp = JsonGeneralResponse::make_response(
                    &req.request(),
                    &StatusCode::UNAUTHORIZED,
                    "Access token is required",
                );
                return Box::pin(async move { Ok(req.into_response(resp.map_into_boxed_body())) });
            }
            Ok(Some(auth_header_value)) => {
                if let Some(at) = auth_header_value.strip_prefix("Bearer ") {
                    at.to_string()
                } else {
                    let resp = JsonGeneralResponse::make_response(
                        &req.request(),
                        &StatusCode::UNAUTHORIZED,
                        "Invalid authorization format. Expected 'Bearer <token>'",
                    );
                    return Box::pin(
                        async move { Ok(req.into_response(resp.map_into_boxed_body())) },
                    );
                }
            }
        };

        // validate token
        let token_data = match decode_access_token(&access_token) {
            Err(e) => {
                // check if path is refresh endpoint
                if req.path().eq_ignore_ascii_case("/api/auth/refresh")
                    && e.to_string().eq_ignore_ascii_case("expiredsignature")
                {
                    match decode_access_token_no_validation_exp(&access_token) {
                        Err(er) => {
                            log::error!("{}", er);
                            let resp = JsonGeneralResponse::make_response(
                                &req.request(),
                                &StatusCode::INTERNAL_SERVER_ERROR,
                                "Server error, try again later.",
                            );

                            return Box::pin(async move {
                                Ok(req.into_response(resp.map_into_boxed_body()))
                            });
                        }
                        Ok(td) => {
                            return _let_through(service, req, &td.claims.sub);
                        }
                    }
                } else if e.to_string().eq_ignore_ascii_case("expiredsignature")
                    || e.to_string().eq_ignore_ascii_case("invalidsignature")
                    || e.to_string().eq_ignore_ascii_case("invalidtoken")
                    || e.to_string().starts_with("Base64")
                {
                    let msg = format!("Access token {}", e);
                    let resp = JsonGeneralResponse::make_response(
                        &req.request(),
                        &StatusCode::UNAUTHORIZED,
                        &msg,
                    );
                    return Box::pin(
                        async move { Ok(req.into_response(resp.map_into_boxed_body())) },
                    );
                } else {
                    log::error!("{}", e);
                    let resp = JsonGeneralResponse::make_response(
                        &req.request(),
                        &StatusCode::INTERNAL_SERVER_ERROR,
                        "Server error, try again later.",
                    );

                    return Box::pin(
                        async move { Ok(req.into_response(resp.map_into_boxed_body())) },
                    );
                }
            }
            Ok(token_data) => token_data,
        };

        _let_through(service, req, &token_data.claims.sub)

        // insert claimsub (user's auth identity) into request extension
        // req.extensions_mut().insert(token_data.claims.sub.clone());

        // Box::pin(async move {
        //     let res = service.call(req).await?;
        //     Ok(res)
        // })
    }
}

fn _let_through<S>(
    service: Rc<S>,
    req: ServiceRequest,
    sub: &str,
) -> LocalBoxFuture<'static, Result<ServiceResponse<BoxBody>, Error>>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    req.extensions_mut().insert(sub.to_string());
    Box::pin(async move {
        let res = service.call(req).await?;
        Ok(res)
    })
}
