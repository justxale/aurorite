use argon2::password_hash::{Error, SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub fn hash_password(provided: String) -> Result<Vec<u8>, Error> {
    match Argon2::default().hash_password(provided.as_bytes(), &SaltString::generate(&mut OsRng)) {
        Ok(result) => Ok(Vec::from(result.to_string().as_bytes())),
        Err(e) => Err(e),
    }
}

pub fn verify(provided: &String, hash: &String) -> bool {
    Argon2::default()
        .verify_password(
            provided.as_bytes(),
            &PasswordHash::new(hash).unwrap(),
        )
        .is_ok()
}
