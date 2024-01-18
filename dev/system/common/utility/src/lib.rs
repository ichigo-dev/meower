//------------------------------------------------------------------------------
//! Utility functions.
//------------------------------------------------------------------------------

use argon2::{ Argon2, PasswordHash, PasswordHasher, PasswordVerifier };
use argon2::password_hash::SaltString;
use rand::{ Rng, thread_rng };
use rand::distributions::{ DistString, Alphanumeric };


//------------------------------------------------------------------------------
/// Gets a random string.
//------------------------------------------------------------------------------
pub fn get_random_str( length: usize ) -> String
{
    let mut rng = thread_rng();
    Alphanumeric.sample_string(&mut rng, length)
}


//------------------------------------------------------------------------------
/// Gets a random code.
//------------------------------------------------------------------------------
pub fn get_random_code( length: usize ) -> String
{
    (0..length).map(|_| thread_rng().gen_range(0..10).to_string()).collect()
}


//------------------------------------------------------------------------------
/// Hashes a value.
//------------------------------------------------------------------------------
pub fn hash_value( value: &str ) -> String
{
    let bin = value.as_bytes();
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
/// Verifies a value.
//------------------------------------------------------------------------------
pub fn verify_value( value: &str, hash: &str ) -> bool
{
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    Argon2::default().verify_password(value.as_bytes(), &parsed_hash).is_ok()
}


//------------------------------------------------------------------------------
/// Checks if a value is hashed.
//------------------------------------------------------------------------------
pub fn is_hashed( value: &str ) -> bool
{
    PasswordHash::new(value).is_ok()
}
