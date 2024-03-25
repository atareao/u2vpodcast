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

use super::CustomResponse;

#[derive(Debug)]
pub struct Error{
    details: String,
    status_code: Option<StatusCode>,
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
    pub fn new(msg: &str) -> Self{
        Error{
            details: msg.to_string(),
            status_code: None,
        }
    }

    pub fn new_with_status_code(msg: &str, status_code: StatusCode) -> Self{
        Error{
            details: msg.to_string(),
            status_code: Some(status_code),
        }
    }

    pub fn status_code(&self) -> StatusCode{
        match self.status_code{
            Some(status_code) => status_code,
            None => StatusCode::INTERNAL_SERVER_ERROR,
        }

    }
}

impl ResponseError for Error{
    fn error_response(&self) -> HttpResponse {
        let response: CustomResponse<Option<String>> = CustomResponse::new(
            self.status_code(),
            &self.details,
            None
        );
        HttpResponse::build(self.status_code())
            .json(response)
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
        Error::new(&error.to_string())
    }
}

impl From<IoError> for Error{
    fn from(error: IoError) -> Self{
        Error::new(&error.to_string())
    }
}

impl From<ParseIntError> for Error{
    fn from(error: ParseIntError) -> Self{
        Error::new(&error.to_string())
    }
}

impl From<Utf8Error> for Error{
    fn from(error: Utf8Error) -> Self{
        Error::new(&error.to_string())
    }
}

impl From<MigrateError> for Error{
    fn from(error: MigrateError) -> Self{
        Error::new(&error.to_string())
    }
}

impl From<ActixError> for Error{
    fn from(error: ActixError) -> Self{
        Error::new(&error.to_string())
    }
}
