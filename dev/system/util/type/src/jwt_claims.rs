//------------------------------------------------------------------------------
//! JWTClaims.
//------------------------------------------------------------------------------

use serde::{ Serialize, Deserialize };

pub const JWT_CLAIMS_KEY: &str = "token";


//------------------------------------------------------------------------------
/// JwtClaims.
//------------------------------------------------------------------------------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JwtClaims
{
    pub iss: String,
    pub sub: String,
    pub aud: Vec<String>,
    pub exp: i64,
    pub nbf: i64,
    pub iat: i64,
    pub jti: String,

    // Logined user account name.
    pub uan: String,
}
