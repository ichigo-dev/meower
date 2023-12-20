//------------------------------------------------------------------------------
//! Field hash and verify utility.
//------------------------------------------------------------------------------

use meower_core::Config;

use argon2::{ Argon2, PasswordHash, PasswordHasher, PasswordVerifier };
use argon2::password_hash::SaltString;


//------------------------------------------------------------------------------
/// FieldHash.
//------------------------------------------------------------------------------
pub trait FieldHash
{
    //--------------------------------------------------------------------------
    /// Gets hashed field.
    //--------------------------------------------------------------------------
    fn get_hash_field( &self ) -> String;

    //--------------------------------------------------------------------------
    /// Sets hashed field.
    //--------------------------------------------------------------------------
    fn set_hash_field( &mut self, hash: &str );

    //--------------------------------------------------------------------------
    /// Hashes field.
    //--------------------------------------------------------------------------
    fn hash_field( &mut self )
    {
        let config = Config::new();
        let field = self.get_hash_field();
        let bin = field.as_bytes();
        let salt = SaltString::from_b64(config.get("argon2.phc_salt").as_ref())
            .unwrap();
        let argon2 = Argon2::new
        (
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::default(),
        );
        let hashed_field = argon2.hash_password(bin, &salt)
            .unwrap()
            .to_string();
        self.set_hash_field(&hashed_field);
    }
}


//------------------------------------------------------------------------------
/// FieldVerify.
//------------------------------------------------------------------------------
pub trait FieldVerify
{
    //--------------------------------------------------------------------------
    /// Gets hashed field.
    //--------------------------------------------------------------------------
    fn get_hash_field( &self ) -> String;

    //--------------------------------------------------------------------------
    /// Verifies field.
    //--------------------------------------------------------------------------
    fn verify_field( &self, value: &str ) -> bool
    {
        let field = self.get_hash_field();
        let parsed_hash = PasswordHash::new(&field).unwrap();
        Argon2::default()
            .verify_password(value.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
