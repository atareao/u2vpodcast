use serde_json::Value;
use actix_web::{
    http::StatusCode,
    HttpResponse,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use actix_session::Session;

use super::user::SessionUser;
use super::user::from_session;

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomResponse<T> {
    pub status: bool,
    pub status_code: u16,
    pub message: String,
    pub user: Option<SessionUser>,
    pub data: Option<T>,
}

pub struct CResponse;

impl CResponse {
    pub fn ok(session: Session, data: impl Serialize) -> HttpResponse{
        let content = serde_json::to_value(data).unwrap();
        let response: CustomResponse<Value> = CustomResponse::new(
            StatusCode::OK, "Ok", session, Some(content));
        HttpResponse::build(StatusCode::OK)
            .json(response)
    }

    pub fn purge() -> HttpResponse{
        let response : CustomResponse<String> = CustomResponse {
            status: true,
            status_code: 200,
            message: "Ok".to_string(),
            user: None,
            data: None,
        };
        HttpResponse::build(StatusCode::OK)
            .json(response)
    }

    pub fn ko(status_code: StatusCode, session: Session) -> HttpResponse{
        let user = from_session(session).ok();
        let response = CustomResponse::<Value>{
            status: status_code.is_success(),
            status_code: status_code.as_u16(),
            message: status_code.as_str().to_string(),
            user,
            data: None::<Value>,
        };
        HttpResponse::build(StatusCode::OK)
            .json(response)
    }
}

impl<T> CustomResponse<T> {
    pub fn new(status_code: StatusCode, message: &str, session: Session, data: Option<T>) -> CustomResponse<T>{
        let status_code =  status_code.as_u16();
        let status = status_code < 300;
        let user = from_session(session).ok();
        Self{
            status,
            status_code,
            message: message.to_string(),
            user,
            data,
        }
    }
}

//impl<T> Into<HttpResponse> for CustomResponse<T>
//where T: DeserializeOwned + Serialize{
//    fn into(self) -> HttpResponse {
//        HttpResponse::build(StatusCode::from_u16(self.status_code).unwrap())
//            .json(self)
//    }
//}


impl<T> From<CustomResponse<T>> for HttpResponse {
    fn from(custom_response: CustomResponse<T>) -> HttpResponse{
        custom_response.into()
    }
}
