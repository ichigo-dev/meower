//------------------------------------------------------------------------------
//! Module for user authentication.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth };

use axum::response::{ Response, Redirect };
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
    next: Next,
) -> Result<Response<Body>, Redirect>
{
    let config = state.config();

    // Initializes the authentication.
    let auth = Auth::init_from_cookie(&cookie, config);
    let path= req.uri().path();

    // Skips the authentication for assets and create account page.
    let is_assets = path.starts_with("/_assets");
    if is_assets
    {
        return Ok(next.run(req).await);
    }

    // If not logined, redirects to the login page.
    let is_logined = auth.is_logined();
    let is_auth_page = path.starts_with("/auth");
    if !is_logined && !is_auth_page
    {
        return Err(Redirect::to("/auth/login"));
    }

    // If logined, redirects to the top page.
    if is_logined
    {
        if path.starts_with("/auth/create_account")
        {
            return Ok(next.run(req).await);
        }
        else if auth.is_account_selected() == false
        {
            // If the user has no account, redirects to the create account page.
            let redirect_path = format!
            (
                "/auth/create_account/{}",
                auth.claims().sub,
            );
            return Err(Redirect::to(&redirect_path));
        }
        else if is_auth_page
        {
            return Err(Redirect::to("/"));
        }
    }

    Ok(next.run(req).await)
}
