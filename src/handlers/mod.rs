mod login;
mod logout;
mod status;
mod root;
mod channels;
mod users;
mod config;


use actix_web::web::{self, route};
use minijinja::{path_loader, Environment};
use tracing::{
    info,
    debug,
};
use once_cell::sync::Lazy;

pub static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    debug!("{:?}", env);
    env.set_loader(path_loader("templates"));
    env
});
use super::{
    middleware::Authentication,
    models::{
        CustomResponse,
        Credentials,
        TokenClaims,
        AppState,
    }
};
use channels::{
    api_channels,
    web_channels,
};
use users::{
    api_users,
    web_users,
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    //let auth = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("")
            .service(
                web::resource("/")
                    .wrap(Authentication)
                    .route(web::get().to(root::get_root))
            )
            .service(web::resource("/logout")
                .route(web::get().to(logout::get_logout))
                )
            .service(
                web::resource("/status")
                    .route(web::get().to(status::get_status)))

            .service(
                web::resource("/login")
                    .route(web::get().to(login::get_login))
                    .route(web::post().to(login::post_login))
            ).service(
                web::scope("/api")
                    .wrap(Authentication)
                    .service(
                        web::scope("/1.0")
                            .configure(api_channels)
                            .configure(api_users)
                )
            ).service(
                web::scope("config")
                    .wrap(Authentication)
                    .configure(web_channels)
                    .configure(web_users)
            ),

    );
}
