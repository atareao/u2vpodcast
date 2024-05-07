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

use crate::models::CResponse;

use super::{
    Credentials,
    AppState,
    super::{
        models::User,
        utils::{
            USER_ID_KEY,
            USER_NAME_KEY,
            USER_ROLE_KEY,
            USER_ACTIVE_KEY,
        }
    }
};

pub async fn get_session(
    session: Session,
) -> impl Responder{
    info!("get_session");
    info!("Session status: {:?}", session.status());
    CResponse::ok(session, "")
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
                session.insert(USER_ACTIVE_KEY, user.active)
                    .expect("`user_active` cannot be inserted into session");
                CResponse::ok(session, "")
            }else{
                error!("Unauthorized");
                CResponse::ko(StatusCode::UNAUTHORIZED, session)
            }
        },
        Err(_) => CResponse::ko(StatusCode::UNAUTHORIZED, session)
    }
}
