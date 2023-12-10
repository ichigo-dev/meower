//------------------------------------------------------------------------------
//! Module for user authentication.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth };

use axum::response::{ IntoResponse, Redirect };
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
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = state.config();

    // Initializes the authentication.
    let auth = Auth::init_from_cookie(&cookie, config);

    let is_logined = auth.is_logined().await;
    let is_auth_page = req.uri().path().starts_with("/auth");
    if !is_logined && !is_auth_page
    {
        return Err(Redirect::to("/auth/login"));
    }

    if is_logined && is_auth_page
    {
        return Err(Redirect::to("/"));
    }

    Ok(next.run(req).await)
}
