//------------------------------------------------------------------------------
//! User authentication.
//------------------------------------------------------------------------------

use crate::Config;

use axum::http::{ Request, header };
use axum::body::Body;
use axum_extra::extract::cookie::{ Cookie, CookieJar, SameSite };
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
use time::Duration as TimeDuration;
use uuid::Uuid;

static JWT_COOKIE_KEY: &str = "token";


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
    pub fn init() -> Self
    {
        Self
        {
            claims: None,
        }
    }

    //--------------------------------------------------------------------------
    /// Initializes the authentication from cookie.
    //--------------------------------------------------------------------------
    pub fn init_from_cookie( cookie: &CookieJar, config: &Config ) -> Self
    {
        let jwt = cookie
            .get(JWT_COOKIE_KEY)
            .map(|cookie| cookie.value().to_string())
            .unwrap_or("".to_string());
        Self::init_from_jwt(&jwt, config)
    }

    //--------------------------------------------------------------------------
    /// Initializes the authentication from header.
    //--------------------------------------------------------------------------
    pub fn init_from_header( req: &Request<Body>, config: &Config ) -> Self
    {
        let jwt = req.headers()
            .get(header::AUTHORIZATION)
            .and_then(|auth_header| auth_header.to_str().ok())
            .and_then(|auth_value|
            {
                if auth_value.starts_with("Bearer ")
                {
                    Some(auth_value[7..].to_owned())
                }
                else
                {
                    None
                }
            })
            .unwrap_or("".to_string());
        Self::init_from_jwt(&jwt, config)
    }

    //--------------------------------------------------------------------------
    /// Initializes the authentication from JWT.
    //--------------------------------------------------------------------------
    pub fn init_from_jwt( jwt: &str, config: &Config ) -> Self
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
        let aud = config
            .jwt_audience()
            .to_vec()
            .iter()
            .map(|aud| aud.to_string())
            .collect();
        let claims = Claims
        {
            jti: Uuid::new_v4().to_string(),
            iss: config.jwt_issue().to_string(),
            sub: config.jwt_subject().to_string(),
            aud,
            iat,
            nbf: iat,
            exp,
        };

        let key = EncodingKey::from_secret(config.jwt_secret().as_ref());
        encode(&header, &claims, &key).unwrap()
    }

    //--------------------------------------------------------------------------
    /// Makes JWT token cookie.
    //--------------------------------------------------------------------------
    pub fn make_jwt_cookie( config: &Config ) -> String
    {
        let jwt = Self::make_jwt(config);
        Cookie::build(JWT_COOKIE_KEY, jwt.to_owned())
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(TimeDuration::seconds(config.jwt_expires()))
            .secure(config.debug_mode() == false)
            .finish()
            .to_string()
    }
}
