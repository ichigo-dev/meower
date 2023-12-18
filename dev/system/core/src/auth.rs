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
use time::{ Duration as TimeDuration, OffsetDateTime };
use uuid::Uuid;

static JWT_COOKIE_KEY: &str = "token";


//------------------------------------------------------------------------------
/// JWT Claims.
//------------------------------------------------------------------------------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JwtClaims
{
    pub jti: String,
    pub iss: String,
    pub sub: String,
    pub aud: Vec<String>,
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,

    /// Logined user_account_name
    pub uan: String,
}

impl JwtClaims
{
    //--------------------------------------------------------------------------
    /// Initializes from config
    //--------------------------------------------------------------------------
    pub fn init_from_config( config: &Config ) -> Self
    {
        let now = Utc::now();
        let iat = now.timestamp();
        let jwt_expire_sec = config.get_as_isize("jwt.expire_sec");
        let exp = (now + Duration::seconds(jwt_expire_sec as i64)).timestamp();
        let aud = config
            .get_as_vec("jwt.audience")
            .iter()
            .map(|aud| aud.to_string())
            .collect();

        Self
        {
            jti: Uuid::new_v4().to_string(),
            iss: config.get("jwt.issue"),
            aud,
            iat,
            nbf: iat,
            exp,
            ..Default::default()
        }
    }

    //--------------------------------------------------------------------------
    /// Initializes from cookie.
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
    /// Initializes from header.
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
    /// Initializes from JWT.
    //--------------------------------------------------------------------------
    pub fn init_from_jwt( jwt: &str, config: &Config ) -> Self
    {
        if jwt.is_empty()
        {
            return Self::default();
        }

        // Decodes the JWT token.
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&config.get_as_vec("jwt.audience"));
        match decode::<Self>
        (
            jwt,
            &DecodingKey::from_secret(config.get("jwt.secret").as_ref()),
            &validation,
        )
        {
            Ok(token) => token.claims,
            Err(_) => Self::default(),
        }
    }

    //--------------------------------------------------------------------------
    /// Makes JWT token.
    //--------------------------------------------------------------------------
    pub fn encode( &self, config: &Config ) -> String
    {
        let mut header = Header::default();
        header.typ = Some("JWT".to_string());
        header.alg = Algorithm::HS256;

        let key = EncodingKey::from_secret(config.get("jwt.secret").as_ref());
        encode(&header, &self, &key).unwrap()
    }
}


//------------------------------------------------------------------------------
/// Authentication.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Auth
{
    claims: JwtClaims,
}

impl Auth
{
    //--------------------------------------------------------------------------
    /// Initializes the authentication.
    //--------------------------------------------------------------------------
    pub fn init( claims: JwtClaims ) -> Self
    {
        Self { claims }
    }

    //--------------------------------------------------------------------------
    /// Initializes the authentication from config.
    //--------------------------------------------------------------------------
    pub fn init_from_config( config: &Config ) -> Self
    {
        Self { claims: JwtClaims::init_from_config(config) }
    }

    //--------------------------------------------------------------------------
    /// Initializes the authentication from cookie.
    //--------------------------------------------------------------------------
    pub fn init_from_cookie( cookie: &CookieJar, config: &Config ) -> Self
    {
        Self { claims: JwtClaims::init_from_cookie(cookie, config) }
    }

    //--------------------------------------------------------------------------
    /// Initializes the authentication from header.
    //--------------------------------------------------------------------------
    pub fn init_from_header( req: &Request<Body>, config: &Config ) -> Self
    {
        Self { claims: JwtClaims::init_from_header(req, config) }
    }

    //--------------------------------------------------------------------------
    /// Checks if the user is logged in.
    //--------------------------------------------------------------------------
    pub fn is_logined( &self ) -> bool
    {
        self.claims.sub.len() > 0
    }

    //--------------------------------------------------------------------------
    /// Checks if account is selected.
    //--------------------------------------------------------------------------
    pub fn is_account_selected( &self ) -> bool
    {
        self.claims.uan.len() > 0
    }

    //--------------------------------------------------------------------------
    /// Makes JWT token cookie.
    //--------------------------------------------------------------------------
    pub fn make_jwt_cookie( &self, config: &Config ) -> String
    {
        let now = OffsetDateTime::now_utc();
        let jwt = self.claims.encode(config);
        let jwt_expire = config.get_as_isize("jwt.expire_sec") as i64;
        let jwt_expire_date = now + TimeDuration::seconds(jwt_expire);
        Cookie::build((JWT_COOKIE_KEY, jwt.to_owned()))
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(TimeDuration::seconds(jwt_expire))
            .expires(jwt_expire_date)
            .secure(config.get_as_bool("system.debug_mode") == false)
            .to_string()
    }

    //--------------------------------------------------------------------------
    /// Gets the claims.
    //--------------------------------------------------------------------------
    pub fn claims( &self ) -> &JwtClaims
    {
        &self.claims
    }
}
