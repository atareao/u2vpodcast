use serde_json::{json, Value};
use actix_web::{
    Responder,
    http::StatusCode,
    web::{
        Json,
        Data,
    },
};
use actix_session::Session;
use tracing::{info, error};

use super::{
    Credentials,
    AppState,
    super::{
        models::{
            User,
            CustomResponse,
        },
        utils::{
            USER_ID_KEY,
            USER_NAME_KEY,
            USER_ROLE_KEY,
        }
    }
};

pub async fn get_session(
    session: Session,
) -> impl Responder{
    info!("get_session");
    info!("Session status: {:?}", session.status());
    let id = session.get(USER_ID_KEY).unwrap().unwrap_or(0);
    let name = session.get(USER_NAME_KEY).unwrap().unwrap_or("".to_string());
    let role = session.get(USER_ROLE_KEY).unwrap().unwrap_or("".to_string());
                Json(CustomResponse::new(
                    StatusCode::OK,
                    "Authorized",
                    Some(json!({
                        "id": id,
                        "name": name,
                        "role": role,
                    }))),
                )
}

pub async fn post_login(
    data: Data<AppState>,
    Json(credentials): Json<Credentials>,
    session: Session,
) -> impl Responder{
    info!("post_login");
    match User::get_by_name(&data.pool, &credentials.username).await{
        Ok(user) => {
            if user.active && user.check_password(&credentials.password).await {
                info!("Ok");
                session.renew();
                session.insert(USER_ID_KEY, user.id)
                    .expect("`user_id` cannot be inserted into session");
                session.insert(USER_NAME_KEY, &user.name)
                    .expect("`user_name` cannot be inserted into session");
                session.insert(USER_ROLE_KEY, &user.role)
                    .expect("`user_role` cannot be inserted into session");
                Json(CustomResponse::new(
                    StatusCode::OK,
                    "Authorized",
                    Some(json!({
                        "id": user.id,
                        "name": user.name,
                        "role": user.role,
                        "active": user.active,
                    }))),
                )
            }else{
                let response: CustomResponse<Option<Value>> = CustomResponse::new(
                        StatusCode::UNAUTHORIZED,
                        "Authorized",
                        None);
                error!("Unauthorized");
                Json(response)
            }
        },
        Err(e) => {
                let response: CustomResponse<Option<Value>> = CustomResponse::new(
                        StatusCode::UNAUTHORIZED,
                        &format!("Authorized: {e}"),
                        None);
                error!("Not found");
                Json(response)
        }
    }
}
