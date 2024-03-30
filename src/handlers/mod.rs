mod login;
mod logout;
mod status;
mod channels;
mod episodes;
mod users;
mod options;


use actix_web::web;
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
    config_channels,
    web_channels,
};
use episodes::web_episodes;
use users::{
    api_users,
    config_users,
};
use options::{
    api_options,
    config_options,
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    //let auth = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("")
            .service(
                web::redirect("/", "/channels")
            )
            .configure(web_channels)
            .configure(web_episodes)
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
                    //.wrap(Authentication)
                    .service(
                        web::scope("/1.0")
                            .configure(api_channels)
                            .configure(api_users)
                            .configure(api_options)
                )
            ).service(
                web::scope("/config")
                    .wrap(Authentication)
                    .configure(config_channels)
                    .configure(config_users)
                    .configure(config_options)
            )

    );
}
