//------------------------------------------------------------------------------
//! Protected layer.
//------------------------------------------------------------------------------

use crate::{ AppState, Config };

use std::fs::File;
use std::io::Read;

use meower_auth_shared::JwtClaims;
use axum::body::Body;
use axum::extract::State;
use axum::http::{ header, StatusCode };
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{ IntoResponse, Response };
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{
    decode,
    Algorithm,
    DecodingKey,
    Validation,
};


//------------------------------------------------------------------------------
/// Layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    state: State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = &state.config;

    let cookie = CookieJar::from_headers(&req.headers());
    let access_token = cookie
        .get(&config.jwt_access_token_key)
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_default();
    if access_token.len() <= 0
    {
        return Err(redirect_to_auth(config));
    }

    let key_path = "./env/".to_string() + &config.jwt_public_key;
    let mut key_data = String::new();
    let mut file = File::open(&key_path).unwrap();
    file.read_to_string(&mut key_data).unwrap();

    let key = DecodingKey::from_rsa_pem(key_data.as_bytes()).unwrap();
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&config.client_id]);
    let _access_token = match decode::<JwtClaims>
    (
        &access_token,
        &key,
        &validation,
    )
    {
        Ok(token) => token,
        Err(_) => return Err(redirect_to_auth(config)),
    };

    Ok(next.run(req).await)
}

// Redirects to the authentication server.
fn redirect_to_auth( config: &Config ) -> impl IntoResponse
{
    let url = format!
    (
        "{}?{}={}",
        config.auth_login_url,
        config.client_id_key,
        config.client_id,
    );
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, url)
        .body(Body::empty())
        .unwrap();
    response
}
