use minijinja::context;
use actix_web::{
    HttpResponse,
    Responder,
    web::Data,
};
use tracing::info;

use super::{
    ENV,
    AppState
};

pub async fn get_root(data: Data<AppState>) -> impl Responder{
    info!("get_root");
    let config = &data.config;
    let template = ENV.get_template("index.html").unwrap();
    let ctx = context! {
        title => config.title,
    };
    HttpResponse::Ok().body(template.render(ctx).unwrap())
}


