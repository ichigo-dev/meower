//------------------------------------------------------------------------------
//! Module for user authentication.
//------------------------------------------------------------------------------

use crate::Config;

use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
};
use chrono::{ Utc, Duration };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;


//------------------------------------------------------------------------------
/// JWT Claims.
//------------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
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
pub(crate) struct Auth
{
}

impl Auth
{
    //--------------------------------------------------------------------------
    /// Initializes the authentication.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        Self {}
    }

    //--------------------------------------------------------------------------
    /// Logs in the user.
    //--------------------------------------------------------------------------
    pub(crate) async fn login( &self, _email: &str, _password: &str ) -> bool
    {
        true
    }

    //--------------------------------------------------------------------------
    /// Checks if the user is logged in.
    //--------------------------------------------------------------------------
    pub(crate) async fn is_logined( &self ) -> bool
    {
        true
    }

    //--------------------------------------------------------------------------
    /// Makes JWT token.
    //--------------------------------------------------------------------------
    pub(crate) fn make_jwt( &self, config: &Config ) -> String
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
}
