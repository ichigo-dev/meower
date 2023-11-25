//------------------------------------------------------------------------------
//! User authentication.
//------------------------------------------------------------------------------

use crate::Config;

use argon2::{ self, Argon2, PasswordHash, PasswordHasher, PasswordVerifier };
use argon2::password_hash::SaltString;
use jsonwebtoken::{
    encode,
    decode,
    Header,
    Algorithm,
    EncodingKey,
    DecodingKey,
    Validation,
};
use chrono::{ Utc, Duration };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;


//------------------------------------------------------------------------------
/// JWT Claims.
//------------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Claims
{
    jti: String,
    iss: String,
    sub: String,
    aud: Vec<String>,
    iat: i64,
    nbf: i64,
    exp: i64,
}


//------------------------------------------------------------------------------
/// Authentication.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Auth
{
    claims: Option<Claims>,
}

impl Auth
{
    //--------------------------------------------------------------------------
    /// Initializes the authentication.
    //--------------------------------------------------------------------------
    pub fn init( jwt: &str, config: &Config ) -> Self
    {
        if jwt.is_empty()
        {
            return Self
            {
                claims: None,
            };
        }

        // Decodes the JWT token.
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&config.jwt_audience());
        match decode::<Claims>
        (
            jwt,
            &DecodingKey::from_secret(config.jwt_secret().as_ref()),
            &validation,
        )
        {
            Ok(token) =>
            {
                return Self
                {
                    claims: Some(token.claims),
                };
            },
            Err(e) =>
            {
                println!("{:?}", e);
                return Self
                {
                    claims: None,
                };
            },
        }
    }

    //--------------------------------------------------------------------------
    /// Checks if the user is logged in.
    //--------------------------------------------------------------------------
    pub async fn is_logined( &self ) -> bool
    {
        self.claims.is_some()
    }

    //--------------------------------------------------------------------------
    /// Makes JWT token.
    //--------------------------------------------------------------------------
    pub fn make_jwt( config: &Config ) -> String
    {
        let mut header = Header::default();
        header.typ = Some("JWT".to_string());
        header.alg = Algorithm::HS256;

        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::seconds(config.jwt_expires())).timestamp();
        let claims = Claims
        {
            jti: Uuid::new_v4().to_string(),
            iss: config.jwt_issue().to_string(),
            sub: config.jwt_subject().to_string(),
            aud: config.jwt_audience().clone(),
            iat,
            nbf: iat,
            exp,
        };

        let key = EncodingKey::from_secret(config.jwt_secret().as_ref());
        encode(&header, &claims, &key).unwrap()
    }

    //--------------------------------------------------------------------------
    /// Hashes the password.
    //--------------------------------------------------------------------------
    pub fn password_hash( password: &str, config: &Config ) -> String
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
    pub fn password_verify( password: &str, hash: &str ) -> bool
    {
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

