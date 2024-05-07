use std::{
    fmt::{
        Display,
        Formatter,
        Result
    },
    str::Utf8Error,
    error::Error as StdError,
    io::Error as IoError,
    num::ParseIntError,
};
use serde::{Serialize, ser::SerializeStruct};
use sqlx::{Error as SQLxError, migrate::MigrateError};
use actix_web::{
    Error as ActixError,
    HttpResponse,
    http::StatusCode,
    ResponseError,
};
use actix_session::Session;

pub struct Error{
    details: String,
    session: Option<Session>,
    status_code: Option<StatusCode>,
}
use super::super::models::CustomResponse;

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result{
        f.debug_struct("Error")
            .field("details", &self.details)
            .field("status_code", &self.status_code)
            .finish()
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("details", &self.details)?;
        state.serialize_field("status_code", &self.status_code.unwrap().as_u16())?;
        state.end()
    }

}

impl Error{
    pub fn set_session(&mut self, session: Session) {
        self.session = Some(session);
    }
    pub fn default(msg: &str) -> Self{
        Error{
            details: msg.to_string(),
            status_code: None,
            session: None,
        }
    }
    pub fn new(msg: &str, session: &Session) -> Self{
        Error{
            details: msg.to_string(),
            status_code: None,
            session: Some(session.clone()),
        }
    }

    pub fn new_with_status_code(msg: &str, status_code: StatusCode) -> Self{
        Error{
            details: msg.to_string(),
            status_code: Some(status_code),
            session: None,
        }
    }

    pub fn status_code(&self) -> StatusCode{
        match self.status_code{
            Some(status_code) => status_code,
            None => StatusCode::INTERNAL_SERVER_ERROR,
        }

    }
}

impl Display for Error{
    fn fmt(&self, f: &mut Formatter) -> Result{
        write!(f, "{}", self.details)
    }
}


impl StdError for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<SQLxError> for Error{
    fn from(error: SQLxError) -> Self{
        Error::default(&error.to_string())
    }
}

impl From<IoError> for Error{
    fn from(error: IoError) -> Self{
        Error::default(&error.to_string())
    }
}

impl From<ParseIntError> for Error{
    fn from(error: ParseIntError) -> Self{
        Error::default(&error.to_string())
    }
}

impl From<Utf8Error> for Error{
    fn from(error: Utf8Error) -> Self{
        Error::default(&error.to_string())
    }
}

impl From<MigrateError> for Error{
    fn from(error: MigrateError) -> Self{
        Error::default(&error.to_string())
    }
}

impl From<ActixError> for Error{
    fn from(error: ActixError) -> Self{
        Error::default(&error.to_string())
    }
}

impl ResponseError for Error {
     fn error_response(&self) -> HttpResponse {
        let response: CustomResponse<Option<String>> = CustomResponse::new(
            self.status_code(),
            &self.details,
            self.session.clone().unwrap(),
            None,
        );
        HttpResponse::build(self.status_code())
            .json(response)
    }
}
