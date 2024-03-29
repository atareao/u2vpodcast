use serde::{Serialize, Deserialize};
use actix_web::{
    Responder,
    HttpResponse,
    web::{
        Data,
        Json,
        ServiceConfig, self,
    },
    http::StatusCode,
    get,
    post,
};
use tracing::{
    info,
    debug,
    error,
};
use minijinja::{
    context,
    value::Value,
};

use crate::models::Param;

use super::{
    ENV,
    AppState,
    super::models::CustomResponse,
};

pub fn api_options(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("options/")
            .service(post_options)
    );
}

pub fn config_options(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("options")
            .service(get_options)
    );

}

#[derive(Serialize, Deserialize)]
struct KeyValue{
    key: String,
    value: String
}

#[post("/")]
async fn post_options(
    data: Data<AppState>,
    pairs: Json<Vec<KeyValue>>,
) -> impl Responder {
    info!("post_options");
    let mut response_pairs = Vec::new();
    for pair in pairs.into_inner().as_slice(){
        match Param::set(&data.pool, &pair.key, &pair.value).await {
            Ok(kv) => {
                debug!("{:?}", kv);
                let key = kv.get_key(); 
                let value = kv.get_value();
                response_pairs.push(KeyValue{
                    key: key.to_string(),
                    value: value.to_string(),
                });
            },
            Err(e) => {
                error!("{:?}", e);
                response_pairs.push(KeyValue{
                    key: pair.key.clone(),
                    value: pair.value.clone(),
                });
            }
        }
    }
    // TODO: update AppState => Mutex ???
    Json(CustomResponse::new(
        StatusCode::OK,
        "Ok",
        response_pairs,
    ))
}

#[get("/")]
async fn get_options(
    data: Data<AppState>,
) -> impl Responder{
    info!("get_options");
    let config = &data.config;
    let title = &config.title;
    let params = Param::get_all(&data.pool).await.unwrap();
    debug!("{:?}", params);
    let template = ENV.get_template("config/options.html").unwrap();
    let ctx = context! {
        page_title => &format!("{title} - General options"),
        ..Value::from_serializable(&params),
    };
    HttpResponse::Ok().body(template.render(ctx).unwrap())
}
