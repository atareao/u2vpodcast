use serde::Deserialize;
use actix_web::{
    Responder,
    web::{
        Path,
        Data,
        Query,
        Json,
        ServiceConfig, self,
    },
    http::StatusCode,
    get,
    post,
    delete,
};
use tracing::{
    info,
    error,
    debug,
};

use super::{
    AppState,
    super::models::{
        CustomResponse,
        User,
        NewUser,
    },
};

pub fn api_users(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("users/")
            .service(create)
            .service(delete)
            .service(read)
            .service(read_with_pagination)
    );
}

#[derive(Deserialize)]
struct Page{
    page: Option<i64>,
}

#[derive(Deserialize)]
struct Info{
    user_id: i64,
}


#[get("/")]
async fn read_with_pagination(
    data: Data<AppState>,
    page: Query<Page>,
) -> impl Responder{
    info!("read_all");
    let page = page.page.unwrap_or(1);
    let per_page = data.config.per_page;
    match User::read_with_pagination(&data.pool, page, per_page).await{
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            error!("Error: {e}");
            Err(e)
        },
    }
}

#[post("/")]
async fn create(
    data: Data<AppState>,
    user: Json<NewUser>,
) -> impl Responder {
    info!("create");
    match User::new(&data.pool, &data.config, user.into_inner()).await{
            Ok(user) => Ok(Json(CustomResponse::new(
            StatusCode::OK,
            "Ok",
            user,
        ))),
            Err(e) => {
                error!("Error: {e}");
                Err(e)
            },
        }
}

#[get("/{user_id}/")]
async fn read( data: Data<AppState>, path: Path<Info>,) -> impl Responder{
    info!("read");
    match User::read(&data.pool, path.user_id).await{
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            error!("Error: {e}");
            Err(e)
        },
    }
}

#[delete("/")]
async fn delete( data: Data<AppState>, path: Query<Info>,) -> impl Responder{
    info!("delete");
    match User::delete(&data.pool, path.user_id).await{
        Ok(channel) => {
            debug!("{:?}", channel);
            Ok(Json(CustomResponse::new(
                StatusCode::OK,
                "Ok",
                channel,
            )))
        },
        Err(e) => {
            error!("Error: {e}");
            Err(e)
        },
    }
}
