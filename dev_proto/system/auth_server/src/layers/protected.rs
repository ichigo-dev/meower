//------------------------------------------------------------------------------
//! Protected layer.
//------------------------------------------------------------------------------

use crate::state::AppState;
use crate::pages::already_logined::PageTemplate;
use meower_type::{ JwtClaims, JWT_CLAIMS_KEY };

use askama::Template;
use axum::response::{ Html, IntoResponse };
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::extract::State;
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
    State(state): State<AppState>,
    cookie: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = state.config;

    // Gets the JWT claims from the cookie.
    let jwt_claims_cookie = cookie
        .get(JWT_CLAIMS_KEY)
        .map(|cookie| cookie.value().to_string())
        .unwrap_or("".to_string());

    // Validates the JWT claims.
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&config.jwt_audience);
    let result = decode::<JwtClaims>
    (
        &jwt_claims_cookie,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &validation,
    );
    if result.is_ok()
    {
        let template = PageTemplate::default();
        return Err(Html(template.render().unwrap()));
    }

    Ok(next.run(req).await)
}
