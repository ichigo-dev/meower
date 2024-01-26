//------------------------------------------------------------------------------
//! Protected layer.
//------------------------------------------------------------------------------

use crate::{ AppState, Config };

use axum::body::Body;
use axum::extract::State;
use axum::http::{ header, StatusCode };
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{ IntoResponse, Response };
use axum_extra::extract::cookie::CookieJar;


//------------------------------------------------------------------------------
/// Layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    state: State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = &state.config;

    let cookie = CookieJar::from_headers(&req.headers());
    let user_token = cookie
        .get(&config.user_token_key)
        .map(|cookie| cookie.value().to_string())
        .unwrap_or("".to_string());
    if user_token.len() <= 0
    {
        return Err(redirect_to_auth(config));
    }

    req.extensions_mut().insert(user_token);
    Ok(next.run(req).await)
}

// Redirects to the authentication server.
fn redirect_to_auth( config: &Config ) -> impl IntoResponse
{
    let url = format!
    (
        "{}/auth/login?{}={}",
        config.auth_url,
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
