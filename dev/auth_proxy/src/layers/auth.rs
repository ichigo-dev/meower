//------------------------------------------------------------------------------
//! Module for user authentication.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth, JWT_COOKIE_KEY };

use axum::response::IntoResponse;
use axum::body::Body;
use axum::http::{ header, Request };
use axum::middleware::Next;
use axum::extract::State;
use axum_extra::extract::cookie::CookieJar;


//------------------------------------------------------------------------------
/// Authentication layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
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
