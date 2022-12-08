use axum::{
    Router,
    Extension,
    extract::Multipart,
    routing::{get, post},
    response::{Html, IntoResponse, Response},
    http::StatusCode,
};
use std::collections::HashMap;
use tera::{Tera, Context};

use crate::{http::ApiContext, models::user::User};
use super::{error::Error, extractor::AuthUser};

pub fn router() -> Router {
    Router::new()
        .route("/hello",
            get(hello)
        )
        .route("/signup",
            get(get_signup)
            .post(post_signup)
        )
}

async fn hello(
    ctx: Extension<ApiContext>,
    t: Extension<Tera>
) -> impl IntoResponse{
    let mut context = Context::new();
    context.insert("title", "Hola");
    context.insert("name", "lorenzo");
    Html(t.render("hello.html", &context).unwrap())
}

async fn get_signup(
    ctx: Extension<ApiContext>,
    t: Extension<Tera>
) -> impl IntoResponse{
    let mut context = Context::new();
    context.insert("title", "Signup");
    Html(t.render("signup.html", &context).unwrap())
}

async fn post_signup(
    ctx: Extension<ApiContext>,
    multipart: Multipart,
) -> impl IntoResponse {
    let data = parse_multipart(multipart)
        .await
        .map_err(|err| error_page(&err))?;
    if let (Some(username), Some(password), Some(confirm_password)) = (
        data.get("username"),
        data.get("password"),
        data.get("confirm_password"),
    ) {
        if password != confirm_password {
            return Err(error_page(&Error::PasswordsDoNotMatch));
        }
        let user = User::create(&ctx.pool, username, password).await;
        Ok(todo!())
    } else {
        Err(error_page(&Error::MissingDetails))
    }
}

pub(crate) async fn parse_multipart(
    mut multipart: Multipart,
) -> Result<HashMap<String, String>, Error> {
    let mut map = HashMap::new();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_err| Error::ReadError)?
    {
        let name = field.name().ok_or(Error::NoName)?.to_string();

        let data = field
            .text()
            .await
            .map_err(|_| Error::InvalidValue)?;

        map.insert(name, data);
    }
    Ok(map)
}

pub(crate) fn error_page(err: &dyn std::error::Error) -> impl axum::response::IntoResponse {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Err: {}", err))
        .unwrap()
}
