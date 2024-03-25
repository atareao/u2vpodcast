use minijinja::context;
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
    ENV,
    Credentials,
    TokenClaims,
    AppState,
};

pub async fn get_login(data: Data<AppState>) -> impl Responder{
    info!("get_login");
    let config = &data.config;
    let title = &config.title;
    let template = ENV.get_template("login.html").unwrap();
    let ctx = context! {
        title => &title,
    };
    HttpResponse::Ok().body(template.render(ctx).unwrap())
}

pub async fn post_login(data: Data<AppState>, Form(credentials): Form<Credentials>) -> impl Responder{
    info!("post_login");
    let config = &data.config;
    let title = &config.title;
    match User::get_by_name(&data.pool, &credentials.username).await{
    //match config.get_user(&credentials.username) {
        Ok(user) => {
            if user.active && user.check_password(config, &credentials.password) {
                let token = TokenClaims::generate_token(config.to_owned(), &credentials);
                info!("Ok");
                let cookie = Cookie::build("session_auth", token)
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .finish();
                HttpResponse::SeeOther()
                    .insert_header((header::LOCATION, "/"))
                    .cookie(cookie)
                    .finish()
            }else{
                error!("Invalid credentials");
                let template = ENV.get_template("error.html").unwrap();
                let ctx = context! {
                    title => &title,
                    error_description => "Invalid credentials",
                };
                HttpResponse::NonAuthoritativeInformation()
                    .body(template.render(ctx).unwrap())
            }

        },
        Err(e) => {
            error!("{}", e);
            let template = ENV.get_template("error.html").unwrap();
            let ctx = context! {
                title => &title,
                error_description => e.to_string(),

            };
            HttpResponse::Ok()
                .body(template.render(ctx).unwrap())
        }
    }
}
