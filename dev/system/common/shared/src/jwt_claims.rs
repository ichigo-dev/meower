//------------------------------------------------------------------------------
//! JWT claims.
//------------------------------------------------------------------------------

use serde::{ Serialize, Deserialize };


//------------------------------------------------------------------------------
/// JWT claims.
//------------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims
{
    pub jti: String,
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
    pub nbf: i64,
}
