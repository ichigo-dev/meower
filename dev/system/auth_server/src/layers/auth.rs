//------------------------------------------------------------------------------
//! Module for user authentication.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth };

use axum::response::IntoResponse;
use axum::body::Body;
use axum::http::Request;
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

    // Initializes the authentication.
    let auth = Auth::init_from_cookie(&cookie, config);
    req.extensions_mut().insert(auth);
    next.run(req).await
}
