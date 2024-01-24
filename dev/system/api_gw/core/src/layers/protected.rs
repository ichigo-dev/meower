//------------------------------------------------------------------------------
//! Protected layer.
//------------------------------------------------------------------------------

use crate::AppState;

use std::fs::File;
use std::io::Read;

use meower_auth_shared::JwtClaims;
use axum::body::Body;
use axum::extract::State;
use axum::http::{ header, StatusCode };
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
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

    let access_token = req.headers()
        .get(header::AUTHORIZATION)
        .map(|value| value.to_str().unwrap().to_string())
        .unwrap_or_default();
    let client_id = req.headers()
        .get(&config.client_id_key)
        .map(|value| value.to_str().unwrap().to_string())
        .unwrap_or_default();
    if access_token.len() <= 0 || client_id.len() <= 0
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let key_path = "./env/".to_string() + &config.jwt_public_key;
    let mut key_data = String::new();
    let mut file = File::open(&key_path).unwrap();
    file.read_to_string(&mut key_data).unwrap();

    let key = DecodingKey::from_rsa_pem(key_data.as_bytes()).unwrap();
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&client_id]);
    let _access_token = match decode::<JwtClaims>
    (
        &access_token,
        &key,
        &validation,
    )
    {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    Ok(next.run(req).await)
}
