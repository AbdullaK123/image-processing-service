use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use anyhow::Result;

pub fn hash_password(raw_password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password =
        argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
            .to_string();

    Ok(hashed_password)
}

pub fn verify_password(raw_password: &str, password_hash: &str) -> Result<bool> {
    let argon2 = Argon2::default();
    let parsed_hash =
        PasswordHash::new(password_hash)
            .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
    match argon2.verify_password(raw_password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}