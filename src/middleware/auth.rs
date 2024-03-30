use actix_service::forward_ready;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{self, HeaderName, HeaderValue},
        Method,
    },
    web::Data,
    Error,
    HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use tracing::{debug, info};

use crate::models::AppState;

use super::super::utils::token_utils;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("call");
        let mut authenticate_pass = false;
        // Bypass some account routes
        let mut headers = req.headers().clone();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        }
        if !authenticate_pass {
            if let Some(data) = req.app_data::<Data<AppState>>() {
                if let Some(authen_header) = req.headers().get("Authorization") {
                    info!("Parsing authorization header...");
                    if let Ok(authen_str) = authen_header.to_str() {
                        if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                            info!("Parsing token...");
                            let token = authen_str[6..authen_str.len()].trim();
                            if token_utils::check_token_sync(data, token).is_ok() {
                                authenticate_pass = true;
                            }
                        }
                    }
                } else if let Some(cookie) = req.cookie("session_auth") {
                    debug!("Cookie: {cookie}");
                    let token = cookie.value();
                    debug!("Token: {token}");
                    if token_utils::check_token_sync(data, token).is_ok() {
                        authenticate_pass = true;
                    }
                }
            }
        }
        if !authenticate_pass{
            debug!("No authenticate pass");
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/login/"))
                .finish()
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }
        let res = self.service.call(req);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}

