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
use crate::utils::jwt_utils::decode_access_token;

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
                let err_msg = e.to_string().to_lowercase();
                let resp_msg;
                let status: StatusCode;

                if err_msg == "expiredsignature" {
                    resp_msg = "Access token has expired".to_owned();
                    status = StatusCode::UNAUTHORIZED;
                } else if err_msg == "invalidsignature" {
                    resp_msg = "Access token is contains invalid signature".to_owned();
                    status = StatusCode::UNAUTHORIZED;
                } else if err_msg.starts_with("base64 error") {
                    resp_msg = "Access token is not recognized".to_owned();
                    status = StatusCode::UNAUTHORIZED;
                } else if err_msg == "invalidtoken" {
                    resp_msg = "Access token is invalid".to_owned();
                    status = StatusCode::UNAUTHORIZED;
                } else {
                    resp_msg = "Server error, try again later".to_owned();
                    status = StatusCode::INTERNAL_SERVER_ERROR;
                }

                let resp = JsonGeneralResponse::make_response(&req.request(), &status, &resp_msg);

                return Box::pin(async move { Ok(req.into_response(resp.map_into_boxed_body())) });
            }
            Ok(token_data) => token_data,
        };

        // insert claimsub (user's auth identity) into request extension
        req.extensions_mut().insert(token_data.claims.sub.clone());

        Box::pin(async move {
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}
