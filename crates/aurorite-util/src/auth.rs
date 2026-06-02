use crate::uuid::ALPHABET;
use argon2::password_hash::rand_core::RngCore;
use argon2::password_hash::{Error, SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub fn hash_password(provided: &String) -> Result<String, Error> {
    match Argon2::default().hash_password(provided.as_bytes(), &SaltString::generate(&mut OsRng)) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e),
    }
}

pub fn verify(provided: &String, hash: &str) -> bool {
    Argon2::default()
        .verify_password(provided.as_bytes(), &PasswordHash::new(hash).unwrap())
        .is_ok()
}

pub fn generate_password() -> String {
    let mut pwd: [char; 12] = ['a'; 12];
    for c in pwd.iter_mut() {
        *c = ALPHABET[(OsRng.next_u32() % 64) as usize];
    }
    String::from_iter(pwd)
}
