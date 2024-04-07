use actix_web::{
    HttpResponse,
    Responder,
    http::header,
    web::{
        Form,
        Data,
    },
    cookie::{
        Cookie,
        SameSite,
    },
};
use tracing::{info, error};

use crate::models::User;

use super::{
    super::Error,
    Credentials,
    TokenClaims,
    AppState,
};

pub async fn post_login(data: Data<AppState>, Form(credentials): Form<Credentials>) -> impl Responder{
    info!("post_login");
    let config = &data.config;
    match User::get_by_name(&data.pool, &credentials.username).await{
    //match config.get_user(&credentials.username) {
        Ok(user) => {
            if user.active && user.check_password(config, &credentials.password) {
                let token = TokenClaims::generate_token(config.to_owned(), &user);
                info!("Ok");
                let cookie = Cookie::build("session_auth", token)
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .finish();
                Ok(HttpResponse::SeeOther()
                    .insert_header((header::LOCATION, "/app/configure/channels/"))
                    .cookie(cookie)
                    .finish())
            }else{
                error!("Invalid credentials");
                Err(Error::new("Invalid credentials"))
            }

        },
        Err(e) => {
            error!("{}", e);
            Err(Error::new(&format!("Error: {e}")))
        }
    }
}
