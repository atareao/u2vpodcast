use serde::Deserialize;
use actix_web::{
    Responder,
    HttpResponse,
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
    debug,
};
use minijinja::context;

use super::{
    ENV,
    AppState,
    super::models::{
        CustomResponse,
        User,
        NewUser,
        Role,
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

pub fn web_users(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("users")
            .service(read_web)
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
        Err(e) => Err(e),
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
            Err(e) => Err(e),
        }
}

#[get("/{user_id}")]
async fn read( data: Data<AppState>, path: Path<Info>,) -> impl Responder{
    info!("read");
    match User::read(&data.pool, path.user_id).await{
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}

#[delete("/")]
async fn delete( data: Data<AppState>, path: Query<Info>,) -> impl Responder{
    info!("delete");
    match User::delete(&data.pool, path.user_id).await{
        Ok(channel) => Ok(Json(CustomResponse::new(
            StatusCode::OK,
            "Ok",
            channel,
        ))),
        Err(e) => Err(e),
    }
}


#[get("/")]
async fn read_web(
    data: Data<AppState>,
    page: Query<Page>,
) -> impl Responder{
    info!("read_all");
    let config = &data.config;
    let title = &config.title;
    let per_page = config.per_page;
    let page = page.page.unwrap_or(1);
    match User::read_with_pagination(&data.pool, page, per_page).await{
        Ok(users) => {
            debug!("{:?}", users);
            let template = ENV.get_template("users.html").unwrap();
            let ctx = context! {
                title => &format!("{title} - Configure users"),
                users => users,
                roles => Role::get_roles(),
            };
            HttpResponse::Ok().body(template.render(ctx).unwrap())
        },
        Err(error) => {
            let template = ENV.get_template("error.html").unwrap();
            let ctx = context! {
                title => &title,
                error => error,
            };
            HttpResponse::Ok().body(template.render(ctx).unwrap())
        },
    }
}
