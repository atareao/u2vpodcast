use serde::{Serialize, Deserialize};
use actix_web::{
    Responder,
    web::{
        self,
        Data,
        Json,
        ServiceConfig,
    },
    http::StatusCode,
    post,
    get,
};
use tracing::{
    info,
    debug,
    error,
};

use super::{
    AppState,
    super::{
        utils::worker::do_the_work,
        models::{
            Param,
            CustomResponse
        },
    },
};

pub fn api_options(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("options")
            .service(post_options)
            .service(update)
    );
}

#[derive(Serialize, Deserialize)]
struct KeyValue{
    key: String,
    value: String
}

#[get("update/")]
async fn update(
    data: Data<AppState>,
) -> impl Responder {
    info!("update");
    match do_the_work(&data.pool, false).await{
        Ok(()) => Ok(Json(CustomResponse::new(
            StatusCode::OK,
            "Ok",
            ""
        ))),
        Err(e) => Err(e),
    }
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
