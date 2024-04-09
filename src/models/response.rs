use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomResponse<T> {
    pub status: bool,
    pub status_code: u16,
    pub message: String,
    pub data: T,
}

impl<T> CustomResponse<T> {
    pub fn new(status_code: StatusCode, message: &str, data: T) -> CustomResponse<T>{
        let status_code =  status_code.as_u16();
        let status = status_code < 300;
        Self{
            status,
            status_code,
            message: message.to_string(),
            data,
        }
    }
}
