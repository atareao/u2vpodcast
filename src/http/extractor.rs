use crate::{http::error::Error, config::Configuration};
use axum::{
    async_trait,
    extract::FromRequest,
    body::Bytes,
    response::Response,
    middleware::Next,
    http::{
        header::{
            HeaderValue,
            AUTHORIZATION,
        },
        StatusCode,
        Request,
    },
    Extension,
};
use tower::{Service, Layer};
use std::task::{Context, Poll};

use crate::http::ApiContext;
use crate::models::user::User;
use hmac::{Hmac, digest::KeyInit};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha384;
use time::{Duration, OffsetDateTime};

type HmacSha384 = Hmac<Sha384>;

const DEFAULT_SESSION_LENGTH: Duration = Duration::weeks(2);

// Ideally the Realworld spec would use the `Bearer` scheme as that's relatively standard
// and has parsers available, but it's really not that hard to parse anyway.
const SCHEME_PREFIX: &str = "Token ";

/// Add this as a parameter to a handler function to require the user to be logged in.
///
/// Parses a JWT from the `Authorization: Token <token>` header.
pub struct AuthUser {
    pub id: i64,
}

/// Add this as a parameter to a handler function to optionally check if the user is logged in.
///
/// If the `Authorization` header is absent then this will be `Self(None)`, otherwise it will
/// validate the token.
///
/// This is in contrast to directly using `Option<AuthUser>`, which will be `None` if there
/// is *any* error in deserializing, which isn't exactly what we want.
pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    id: i64,
    /// Standard JWT `exp` claim.
    exp: i64,
}

impl AuthUser {
    pub(in crate::http) fn to_jwt(&self, ctx: &ApiContext) -> String {
        let hmac = HmacSha384::new_from_slice(ctx.config.get_hmac_key().as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        AuthUserClaims {
            id: self.id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC signing should be infallible")
    }

    /// Attempt to parse `Self` from an `Authorization` header.
    fn from_authorization(ctx: &ApiContext, auth_header: &HeaderValue) -> Result<Self, Error> {
        let auth_header = auth_header.to_str().map_err(|_| {
            tracing::debug!("Authorization header is not UTF-8");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            tracing::debug!(
                "Authorization header is using the wrong scheme: {:?}",
                auth_header
            );
            return Err(Error::Unauthorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt =
            jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token).map_err(|e| {
                tracing::debug!(
                    "failed to parse Authorization header {:?}: {}",
                    auth_header,
                    e
                );
                Error::Unauthorized
            })?;

        // Realworld doesn't specify the signing algorithm for use with the JWT tokens
        // so we picked SHA-384 (HS-384) as the HMAC, as it is more difficult to brute-force
        // than SHA-256 (recommended by the JWT spec) at the cost of a slightly larger token.
        let hmac = HmacSha384::new_from_slice(ctx.config.get_hmac_key().as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        // When choosing a JWT implementation, be sure to check that it validates that the signing
        // algorithm declared in the token matches the signing algorithm you're verifying with.
        // The `jwt` crate does.
        let jwt = jwt.verify_with_key(&hmac).map_err(|e| {
            tracing::debug!("JWT failed to verify: {}", e);
            Error::Unauthorized
        })?;

        let (_header, claims) = jwt.into();

        // Because JWTs are stateless, we don't really have any mechanism here to invalidate them
        // besides expiration. You probably want to add more checks, like ensuring the user ID
        // exists and has not been deleted/banned/deactivated.
        //
        // You could also use the user's password hash as part of the keying material for the HMAC,
        // so changing their password invalidates their existing sessions.
        //
        // In practice, Launchbadge has abandoned using JWTs for authenticating long-lived sessions,
        // instead storing session data in Redis, which can be accessed quickly and so adds less
        // overhead to every request compared to hitting Postgres, and allows tracking and
        // invalidating individual sessions by simply deleting them from Redis.
        //
        // Technically, the Realworld spec isn't all that adamant about using JWTs and there
        // may be some flexibility in using other kinds of tokens, depending on whether the frontend
        // is expected to parse the token or just treat it as an opaque string.
        //
        // Also, if the consumer of your API is a browser, you probably want to put your session
        // token in a cookie instead of the response body. By setting the `HttpOnly` flag, the cookie
        // isn't exposed in the response to Javascript at all which, along with setting `Domain` and
        // `SameSite`, prevents all kinds of session hijacking exploits.
        //
        // This also has the benefit of avoiding having to deal with securely storing the session
        // token on the frontend.

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            tracing::debug!("token expired");
            return Err(Error::Unauthorized);
        }

        Ok(Self {
            id: claims.id,
        })
    }
}

impl MaybeAuthUser {
    /// If this is `Self(Some(AuthUser))`, return `AuthUser::user_id`
    pub fn user_id(&self) -> Option<i64> {
        self.0.as_ref().map(|auth_user| auth_user.id)
    }
}

// tower-http has a `RequireAuthorizationLayer` but it's useless for practical applications,
// as it only supports matching Basic or Bearer auth with credentials you provide it.
//
// There's the `::custom()` constructor to provide your own validator but it basically
// requires parsing the `Authorization` header by-hand anyway so you really don't get anything
// out of it that you couldn't write your own middleware for, except with a bunch of extra
// boilerplate.
#[async_trait]
impl<S, B> FromRequest<S, B> for AuthUser
where
    Bytes: FromRequest<S, B>,
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let headers = req.headers().clone();
        let ctx: Extension<ApiContext> = Extension::from_request(req, state)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        // Get the value of the `Authorization` header, if it was sent at all.
        let auth_header = headers
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        Self::from_authorization(&ctx, auth_header)
    }
}
#[async_trait]
impl<S, B> FromRequest<S, B> for MaybeAuthUser 
where
    Bytes: FromRequest<S, B>,
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let headers = req.headers().clone();
        let ctx: Extension<ApiContext> = Extension::from_request(req, state)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        Ok(Self(
            // Get the value of the `Authorization` header, if it was sent at all.
            headers
                .get(AUTHORIZATION)
                .and_then(|auth_header| {
                    Some(AuthUser::from_authorization(&ctx, auth_header))
                })
                .transpose()?,
        ))
    }
}
/*

#[derive(Clone)]
pub struct AuthLayer{
    config: Configuration,
}

#[derive(Clone)]
struct AuthService<S>{
    inner: S,
    config: Configuration,
}

impl<S> Layer<S> for AuthLayer{
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service{
        AuthService { 
            inner,
            config: self.config.clone(),
        }
    }
}
async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_header: HeaderValue) -> Option<User>{
}

impl<S, B> Service<Request<B>> for AuthService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        // Do something with `self.state`.
        //
        // See `axum::RequestExt` for how to run extractors directly from
        // a `Request`.
        let auth_header = req.headers()
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let auth_header = if let Some(auth_header) = auth_header {
            auth_header
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        };

        self.inner.call(req)
    }
}
*/
