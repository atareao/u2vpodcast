use argon2::{
    password_hash::{
        SaltString,
        rand_core::OsRng
    },
    Argon2,
    PasswordHasher,
    PasswordHash,
    PasswordVerifier,
};
use jsonwebtoken::{DecodingKey, Validation};
use base64::{
    engine::general_purpose::STANDARD,
    Engine as _,
};
use tracing::{debug, info, error};

use super::super::models::{
    Error,
    AppState,
    User,
    TokenClaims,
};

pub fn check_token_sync(
    appstate: &AppState,
    token: &str,
) -> Result<bool, Error>{
    futures::executor::block_on(async {
        info!("check_token_sync");
        let secret = STANDARD.encode(&appstate.config.jwt_secret);
        debug!("Secret: {}", secret);
        match jsonwebtoken::decode::<TokenClaims>(
            token,
            &DecodingKey::from_base64_secret(&secret).unwrap(),
            &Validation::default(),
        ){
            Ok(token_data) =>  Ok(User::exists(&appstate.pool, &token_data.claims.sub).await),
            Err(e) =>{
                error!("Error: {}", e);
                Err(Error::new(&e.to_string()))
            }
        }
    })
}

pub async fn hash_password(password: &str) -> String{
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Unable to hash password")
        .to_string()
}

pub async fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error>{
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}
