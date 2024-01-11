//------------------------------------------------------------------------------
//! Protect utility.
//------------------------------------------------------------------------------

use crate::config::Config;
use meower_type::{ JwtClaims, JWT_CLAIMS_KEY };

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{
    decode,
    Algorithm,
    DecodingKey,
    TokenData,
    Validation,
};
use jsonwebtoken::errors::Result;


//------------------------------------------------------------------------------
/// Checks if the user is logined.
//------------------------------------------------------------------------------
pub(crate) fn is_logined
(
    cookie: &CookieJar,
    config: &Config,
) -> Result<TokenData<JwtClaims>>
{
    // Gets the JWT claims from the cookie.
    let jwt_claims_cookie = cookie
        .get(JWT_CLAIMS_KEY)
        .map(|cookie| cookie.value().to_string())
        .unwrap_or("".to_string());

    // Validates the JWT claims.
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&config.jwt_audience);
    decode::<JwtClaims>
    (
        &jwt_claims_cookie,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &validation,
    )
}
