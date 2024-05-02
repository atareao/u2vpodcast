use actix_web::{
    http::StatusCode,
    Responder,
};
use actix_session::Session;
use tracing::{info, error};
use crate::models::CResponse;

use super::super::utils::USER_ID_KEY;

pub async fn get_logout(session: Session) -> impl Responder{
    info!("get_logout");
    match session_user_id(&session).await{
        Ok(_) => {
            info!("Logout");
            session.clear();
            session.purge();
            CResponse::purge()
        },
        Err(e) => {
            error!("Error: {}", e);
            CResponse::ko(StatusCode::BAD_REQUEST, session)
        }
    }
}

async fn session_user_id(session: &actix_session::Session) -> Result<i64, String> {
    match session.get(USER_ID_KEY) {
        Ok(user_id) => match user_id {
            None => Err("You are not authenticated".to_string()),
            Some(id) => Ok(id),
        },
        Err(e) => Err(format!("{e}")),
    }
}

