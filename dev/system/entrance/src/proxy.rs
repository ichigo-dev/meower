//------------------------------------------------------------------------------
//! Proxies requests to the frontend.
//------------------------------------------------------------------------------

use crate::AppState;

use std::env;

use axum::http::Request;
use axum::response::IntoResponse;
use axum::body::Body;
use axum::extract::State;
use hyper::Uri;


//------------------------------------------------------------------------------
/// Proxies requests to the frontend.
//------------------------------------------------------------------------------
pub(crate) async fn handler
(
    State(state): State<AppState>,
    mut req: Request<Body>,
) -> impl IntoResponse
{
    let config = state.config();
    let client = state.client();

    // Proxies the request to the frontend.
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let frontend_url = env::var("FRONTEND_URL").unwrap();
    let uri = format!("{}{}", frontend_url, path_query);
    *req.uri_mut() = Uri::try_from(uri).unwrap();

    client.request(req).await.unwrap()
}
