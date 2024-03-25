use serde::Serialize;
use actix_web::{
    Responder,
    Result,
    web, http::StatusCode,
};
use super::CustomResponse;

#[derive(Serialize)]
struct Response {
    status: bool,
    message: String,
}

pub async fn get_status() -> Result<impl Responder>{
    let response: CustomResponse<Option<String>> = CustomResponse::new(StatusCode::OK, "Up and running", None);
    Ok(web::Json(response))
}

