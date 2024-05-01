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

/// Store the session key prefix as a const so it can't be typo'd anywhere it's used.
const SESSION_KEY_PREFIX: &str = "valid_session_key_for_{}";

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
