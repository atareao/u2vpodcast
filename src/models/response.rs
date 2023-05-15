use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::models::user::User;

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: i64,
    pub email: String,
    pub role: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

impl FilteredUser{
    pub fn from_user(user: &User) -> Self{
        Self {
            id: user.id,
            email: user.email.to_owned(),
            role: user.role.to_owned(),
            verified: user.verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
