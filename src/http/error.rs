use serde::Serialize;
use axum::{
    response::{
        Response,
        IntoResponse
    },
    http::{
        StatusCode,
        header::CONTENT_TYPE,
    },
    Json
};


#[derive(Debug)]
pub enum YTPError {
    ReadError,
    NotFound,
    Unauthorized,
    Sqlx(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse{
    code: u16,
    message: String,
}

impl YTPError{
    fn status_code(&self) -> StatusCode{
        match self{
            Self::ReadError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Sqlx(value) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn get(&self) -> ErrorResponse{
        match self{
            YTPError::ReadError => ErrorResponse{
                code: self.status_code().as_u16(),
                message: "Internal error".to_string(),
            },
            YTPError::NotFound => ErrorResponse{
                code: self.status_code().as_u16(),
                message: "Not found".to_string(),
            },
            YTPError::Unauthorized => ErrorResponse{
                code: self.status_code().as_u16(),
                message: "Unauthorized".to_string(),
            },
            YTPError::Sqlx(message) => ErrorResponse{
                code: self.status_code().as_u16(),
                message: message.to_string(),
            }
        }
    }
    pub fn get_response(&self) -> Response<String>{
        Response::builder()
            .status(self.status_code())
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&self.get()).unwrap())
            .unwrap()

    }
}
impl IntoResponse for YTPError{
    fn into_response(self) -> Response{
        (self.status_code(), Json(self.get())).into_response()
    }
}

