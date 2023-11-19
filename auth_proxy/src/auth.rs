//------------------------------------------------------------------------------
//! Module for user authentication.
//------------------------------------------------------------------------------

use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
};
use chrono::{ Utc, Duration };
use serde::{ Serialize, Deserialize };


//------------------------------------------------------------------------------
/// JWT Claims.
//------------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims
{
    sub: String,
    iat: i64,
    exp: i64,
}


//------------------------------------------------------------------------------
/// Logs in the user.
//------------------------------------------------------------------------------
pub(crate) async fn login( _email: &str, _password: &str ) -> bool
{
    true
}


//------------------------------------------------------------------------------
/// Checks if the user is logged in.
//------------------------------------------------------------------------------
pub(crate) async fn is_logined() -> bool
{
    true
}


//------------------------------------------------------------------------------
/// Makes JWT token.
//------------------------------------------------------------------------------
pub(crate) fn make_jwt( secret: &str, account_id: &str ) -> String
{
    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::HS256;
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(1)).timestamp();
    let claims = Claims
    {
        sub: account_id.to_string(),
        iat,
        exp,
    };

    let key = EncodingKey::from_secret(secret.as_ref());
    encode(&header, &claims, &key).unwrap()
}
