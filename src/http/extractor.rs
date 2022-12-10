use crate::{http::error::Error, config::Configuration};
use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Multipart},
    body::Bytes,
    http::{
        header::{
            HeaderValue,
            AUTHORIZATION,
        },
        Request,
        request::Parts,
        StatusCode,
    },
    Extension, RequestPartsExt,
};
use tower_cookies::Cookies;
use std::collections::HashMap;
use crate::http::error;

use crate::http::ApiContext;
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
#[derive(Debug, Clone)]
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

    fn from_token(ctx: &ApiContext, token: &str) -> Result<Self, Error>{
        let jwt =
            jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token).map_err(|e| {
                tracing::debug!(
                    "failed to parse token {:?}: {}",
                    token,
                    e
                );
                Error::Unauthorized
            })?;
        let hmac = HmacSha384::new_from_slice(ctx.config.get_hmac_key().as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");
        let jwt = jwt.verify_with_key(&hmac).map_err(|e| {
            tracing::debug!("JWT failed to verify: {}", e);
            Error::Unauthorized
        })?;

        let (_header, claims) = jwt.into();
        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            tracing::debug!("token expired");
            return Err(Error::Unauthorized);
        }

        Ok(Self {
            id: claims.id,
        })

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
        Self::from_token(ctx, token)
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
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(ctx)= parts.extract::<Extension<ApiContext>>()
            .await
            .unwrap();
        let auth_header = parts.headers
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

#[derive(Debug)]
pub struct ExtractAuthCookie(AuthUser);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractAuthCookie
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(ctx)= parts.extract::<Extension<ApiContext>>()
            .await
            .unwrap();
        let cookies = Cookies::from_request_parts(parts, state).await?;
        match cookies.get("ytpodcast"){
            Some(cookie) => {
                match AuthUser::from_token(&ctx, cookie.value()){
                    Ok(auth_user) => Ok(ExtractAuthCookie(auth_user)),
                    Err(_) => Err((StatusCode::NON_AUTHORITATIVE_INFORMATION, "Nada"))
                }
            },
            None => Err((StatusCode::NON_AUTHORITATIVE_INFORMATION,
                "`User-Agent` header is missing"))
        }
    }
}

pub(crate) async fn parse_multipart(
    mut multipart: Multipart,
) -> Result<HashMap<String, String>, error::Error> {
    let mut map = HashMap::new();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_err| error::Error::ReadError)?
    {
        let name = field.name().ok_or(error::Error::NoName)?.to_string();

        let data = field
            .text()
            .await
            .map_err(|_| error::Error::InvalidValue)?;

        map.insert(name, data);
    }
    Ok(map)
}
