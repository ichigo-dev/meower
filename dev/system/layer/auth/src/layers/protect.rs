//------------------------------------------------------------------------------
//! Protect layer.
//------------------------------------------------------------------------------

use crate::state::AppState;
use crate::utils::protect::is_logined;

use axum::response::{ IntoResponse, Redirect };
use axum::body::Body;
use axum::http::{ Request, StatusCode };
use axum::middleware::Next;
use axum::extract::State;
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::errors::ErrorKind;


//------------------------------------------------------------------------------
/// Layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    State(state): State<AppState>,
    cookie: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Result<impl IntoResponse, impl IntoResponse>>
{
    // If the user is not logined, then redirect to the login page.
    if let Err(e) = is_logined(&cookie, &state.config)
    {
        match e.kind()
        {
            ErrorKind::ExpiredSignature =>
            {
            }
            _ =>
            {
                if state.config.provide_pages
                {
                    return Err(Ok(Redirect::to("/auth/login")));
                }
                else
                {
                    return Err(Err(StatusCode::UNAUTHORIZED));
                }
            }
        }
    }

    Ok(next.run(req).await)
}
