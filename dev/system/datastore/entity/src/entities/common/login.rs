//------------------------------------------------------------------------------
//! Trait for loginable model.
//------------------------------------------------------------------------------

use meower_core::Config;
use argon2::{ self, Argon2, PasswordHash, PasswordHasher, PasswordVerifier };
use argon2::password_hash::SaltString;
use sea_orm::DbConn;
use async_trait::async_trait;


//------------------------------------------------------------------------------
/// Loginable.
//------------------------------------------------------------------------------
#[async_trait]
pub trait Loginable
{
    async fn try_login( hdb: &DbConn, identifier: &str, password: &str ) -> bool;

    //--------------------------------------------------------------------------
    /// Hashes the password.
    //--------------------------------------------------------------------------
    fn password_hash( password: &str, config: &Config ) -> String
    {
        let bin_password = password.as_bytes();
        let salt = SaltString::from_b64(config.argon2_phc_salt().as_ref())
            .unwrap();
        let argon2 = Argon2::new
        (
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::default(),
        );
        argon2.hash_password(bin_password, &salt)
            .unwrap()
            .to_string()
    }

    //--------------------------------------------------------------------------
    /// Verifies the password.
    //--------------------------------------------------------------------------
    fn password_verify( password: &str, hash: &str ) -> bool
    {
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

