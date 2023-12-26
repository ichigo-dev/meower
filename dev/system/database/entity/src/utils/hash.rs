//------------------------------------------------------------------------------
//! Hashing and verification.
//------------------------------------------------------------------------------

use argon2::{ Argon2, PasswordHash, PasswordHasher, PasswordVerifier };
use argon2::password_hash::SaltString;


//------------------------------------------------------------------------------
/// Hashes a field.
//------------------------------------------------------------------------------
pub fn hash_field( field: &str ) -> String
{
    let bin = field.as_bytes();
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::new
    (
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::default(),
    );
    argon2.hash_password(bin, &salt).unwrap().to_string()
}


//------------------------------------------------------------------------------
/// Verifies a field.
//------------------------------------------------------------------------------
pub fn verify_field( field: &str, hash: &str ) -> bool
{
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    Argon2::default().verify_password(field.as_bytes(), &parsed_hash).is_ok()
}
