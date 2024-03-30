mod login;
mod logout;
mod status;
mod channels;
mod episodes;
mod users;
mod options;
mod feed;


use actix_web::web;
use minijinja::{
    path_loader,
    Environment,
    Error,
    ErrorKind,
    State,
    value::{
        Kwargs,
        Value,
    },
};
use tracing::{
    info,
    debug,
};
use once_cell::sync::Lazy;
use chrono::{
    DateTime,
    FixedOffset,
};
use chrono_tz::Tz;


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
use feed::web_feed;
use users::{
    api_users,
    config_users,
};
use options::{
    api_options,
    config_options,
};

fn value_to_chrono_datetime(
    value: Value,
) -> Result<DateTime<FixedOffset>, Error> {
    match value.as_str(){
        Some(s) => match DateTime::parse_from_rfc3339(s){
            Ok(dt) => Ok(dt),
            Err(e) => Err(Error::new(
                ErrorKind::MissingArgument,
                e.to_string()
            )),
        },
        None => Err(Error::new(
            ErrorKind::MissingArgument,
            "Not a valid string"
        )),
    }
}

pub fn date(_state: &State, value: Value, kwargs: Kwargs) -> Result<String, Error> {
    let format = kwargs.get::<Option<&str>>("format")?;
    match kwargs.get::<Option<&str>>("timezone")?{
        Some(timezone) => {
            let tz: Tz = timezone.parse().unwrap();
            let datetime = value_to_chrono_datetime(value).unwrap().with_timezone(&tz);
            Ok(format!("{}", datetime.format(format.unwrap())))
        },
        None => {
            let datetime = value_to_chrono_datetime(value).unwrap();
            Ok(format!("{}", datetime.format(format.unwrap())))

        },
    }
}

pub static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    debug!("{:?}", env);
    env.set_loader(path_loader("templates"));
    env.add_filter("date", date);
    env
});

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    //let auth = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("")
            .service(
                web::redirect("/", "/channels/")
            )
            .configure(web_channels)
            .configure(web_episodes)
            .configure(web_feed)
            .service(web::resource("/logout/")
                .route(web::get().to(logout::get_logout))
            )
            .service(
                web::resource("/status/")
                    .route(web::get().to(status::get_status)))

            .service(
                web::resource("/login/")
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
