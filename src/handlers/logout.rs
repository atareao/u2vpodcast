use actix_web::{
    cookie::{
        Cookie,
        SameSite
    },
    HttpResponse,
    Responder,
    HttpRequest,
    http::header,
};
use tracing::info;

pub async fn get_logout(_req: HttpRequest) -> impl Responder{
    info!("get_logout");
    let cookie = Cookie::build("session_auth", "")
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish();

    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/login/"))
        .cookie(cookie)
        .finish()
}

