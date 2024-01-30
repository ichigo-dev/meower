//------------------------------------------------------------------------------
//! HTTP forward.
//------------------------------------------------------------------------------

use axum::body::Bytes;
use axum::extract::{ Path, State };
use axum::http::Method;
use axum::response::IntoResponse;
use reqwest::{ Client, Method as ReqMethod };


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------
pub(crate) async fn handler
(
    State(api_url): State<String>,
    Path(path): Path<String>,
    method: Method,
    body: Bytes,
) -> impl IntoResponse
{
    let client = Client::new();
    let method = ReqMethod::from_bytes(method.to_string().as_bytes()).unwrap();
    let uri = format!("{}/{}", api_url, path);

    let res = client
        .request(method, uri)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    res
}
