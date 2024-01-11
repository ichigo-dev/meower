//------------------------------------------------------------------------------
//! JWTClaims.
//------------------------------------------------------------------------------

use serde::{ Serialize, Deserialize };

pub const JWT_CLAIMS_KEY: &str = "token";


//------------------------------------------------------------------------------
/// JwtClaims.
///
/// If you want to get the loggined `user` data, get the `user_id` from the
/// `user_jwt_subject` table using the `sub` claim.
///
/// If you want to get the `user_account` data selected by the user, refer to
/// the `user_account` table using the `uan` claim.
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
