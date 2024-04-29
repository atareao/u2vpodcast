use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use actix_web::{
    Responder,
    http::{header, StatusCode},
    web::{
        Json,
        Data,
    },
};
use tracing::{info, error};

use crate::models::{User, CustomResponse};

use super::{
    Credentials,
    TokenClaims,
    AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccessToken{
    access_token: String,
}

impl AccessToken {
    fn new(access_token: String) -> Self{
        Self{
            access_token,
        }
    }
    
}

pub async fn post_login(data: Data<AppState>, Json(credentials): Json<Credentials>) -> impl Responder{
    info!("post_login");
    let config = &data.config;
    match User::get_by_name(&data.pool, &credentials.username).await{
    //match config.get_user(&credentials.username) {
        Ok(user) => {
            if user.active && user.check_password(config, &credentials.password) {
                let token = TokenClaims::generate_token(config.to_owned(), &user);
                info!("Ok");
                Json(CustomResponse::new(
                    StatusCode::OK,
                    "Authorized",
                    Some(json!({"access_token":token}))),
                )
            }else{
                let response: CustomResponse<Option<Value>> = CustomResponse::new(
                        StatusCode::UNAUTHORIZED,
                        "Authorized",
                        None);
                Json(response)
            }
        },
        Err(e) => {
                let response: CustomResponse<Option<Value>> = CustomResponse::new(
                        StatusCode::UNAUTHORIZED,
                        &format!("Authorized: {e}"),
                        None);
                Json(response)
        }
    }
}
