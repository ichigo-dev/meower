//------------------------------------------------------------------------------
//! CORS layer.
//------------------------------------------------------------------------------

use crate::AppState;

use axum::body::Body;
use axum::extract::State;
use axum::http::header;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;


//------------------------------------------------------------------------------
/// Layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    state: State<AppState>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse
{
    let cors_origins = state.cors_origins.read().await;
    let origin = req
        .headers()
        .get(header::ORIGIN)
        .unwrap_or(&"".parse().unwrap())
        .clone();

    let mut res = next.run(req).await;

    if let Some(origin) = origin.to_str().ok()
    {
        if cors_origins.contains(&origin.to_string())
        {
            res.headers_mut().insert
            (
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                origin.to_string().parse().unwrap()
            );
        }
    }
    res
}
