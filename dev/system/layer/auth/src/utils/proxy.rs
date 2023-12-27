//------------------------------------------------------------------------------
//! Proxies requests to the other.
//------------------------------------------------------------------------------

use crate::AppState;

use axum::http::Request;
use axum::response::IntoResponse;
use axum::body::Body;
use axum::extract::State;
use hyper::Uri;
use hyper_util::rt::TokioExecutor;
use hyper_util::client::legacy::connect::HttpConnector;


//------------------------------------------------------------------------------
/// Proxy handler.
//------------------------------------------------------------------------------
pub(crate) async fn handler
(
    State(state): State<AppState>,
    mut req: Request<Body>,
) -> impl IntoResponse
{
    // Creates the client.
    let client = hyper_util::client::legacy::Client::<(), ()>::builder
    (
        TokioExecutor::new()
    )
        .build(HttpConnector::new());

    // Proxies the request.
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path)
        .trim_start_matches("/");
    let uri = format!("{}/{}", state.config.proxy_url, path_query);
    *req.uri_mut() = Uri::try_from(uri).unwrap();
    client.request(req).await.unwrap()
}
