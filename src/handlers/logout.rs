use actix_web::{
    cookie::{
        Cookie,
        SameSite
    },
    web::Json,
    http::StatusCode,
    HttpResponse,
    Responder,
    HttpRequest,
    http::header,
};
use actix_session::Session;
use tracing::{info, error};
use crate::models::CustomResponse;

use super::super::utils::USER_ID_KEY;

pub async fn get_logout(session: Session) -> impl Responder{
    info!("get_logout");
    match session_user_id(&session).await{
        Ok(_) => {
            info!("Logout");
            Json(CustomResponse::new(
                StatusCode::OK,
                "You have succesfully logged out",
                Some(""),
            ))
        },
        Err(e) => {
            error!("Error: {}", e);
            Json(CustomResponse::new(
                StatusCode::BAD_REQUEST,
                "We currently have some issues. Kindly try again and ensure you are logged in",
                Some(""),
            ))
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

