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
