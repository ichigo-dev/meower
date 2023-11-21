//------------------------------------------------------------------------------
//! Module for user authentication.
//------------------------------------------------------------------------------

use crate::{ AppState, Config, JWT_COOKIE_KEY };

use axum::response::IntoResponse;
use axum::body::Body;
use axum::http::{ header, Request };
use axum::middleware::Next;
use axum::extract::State;
use axum_extra::extract::cookie::CookieJar;
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
pub(crate) struct Auth
{
    claims: Option<Claims>,
}

impl Auth
{
    //--------------------------------------------------------------------------
    /// Initializes the authentication.
    //--------------------------------------------------------------------------
    pub(crate) fn init( jwt: &str, config: &Config ) -> Self
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
        self.claims.is_some()
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


//------------------------------------------------------------------------------
/// Authentication layer.
//------------------------------------------------------------------------------
pub(crate) async fn auth_layer
(
    State(state): State<AppState>,
    cookie: CookieJar,
    mut req: Request<Body>,
    next: Next<Body>,
) -> impl IntoResponse
{
    let config = state.config();

    // Gets the JWT token.
    let jwt = cookie
        .get(JWT_COOKIE_KEY)
        .map(|cookie| cookie.value().to_string())
        .or_else(||
        {
            req.headers()
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
        })
        .unwrap_or("".to_string());

    // Initializes the authentication.
    let auth = Auth::init(&jwt, config);
    req.extensions_mut().insert(auth);
    next.run(req).await
}
