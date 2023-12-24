//------------------------------------------------------------------------------
//! JWTClaim.
//------------------------------------------------------------------------------

use serde::{ Serialize, Deserialize };

pub const JWT_CLAIM_KEY: &str = "jwt_claim";


//------------------------------------------------------------------------------
/// JwtClaim.
//------------------------------------------------------------------------------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JwtClaim {
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
