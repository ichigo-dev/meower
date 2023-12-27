//------------------------------------------------------------------------------
//! Protect auth page layer.
//------------------------------------------------------------------------------

use crate::state::AppState;
use crate::utils::protect::is_logined;

use axum::response::{ IntoResponse, Redirect };
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::extract::State;
use axum_extra::extract::cookie::CookieJar;


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
    // If the user is logined, then redirect to the home page.
    if is_logined(&cookie, &state.config)
    {
        return Err(Redirect::to("/"));
    };

    Ok(next.run(req).await)
}
