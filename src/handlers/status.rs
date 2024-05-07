use actix_session::Session;
use serde::Serialize;
use actix_web::Responder;
use crate::models::CResponse;


#[derive(Serialize)]
struct Response {
    status: bool,
    message: String,
}

pub async fn get_status(
    session: Session
) -> impl Responder{
    CResponse::ok(session, "Up and running")
}

