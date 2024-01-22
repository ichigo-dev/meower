//------------------------------------------------------------------------------
//! Protected layer.
//------------------------------------------------------------------------------

use crate::AppState;

use axum::body::Body;
use axum::extract::State;
use axum::http::{ header, StatusCode };
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{ IntoResponse, Response };


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
    if true
    {
        let config = &state.config;
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
        return Err(response);
    }

    Ok(next.run(req).await)
}
