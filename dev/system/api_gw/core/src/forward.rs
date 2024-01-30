//------------------------------------------------------------------------------
//! HTTP forward.
//------------------------------------------------------------------------------

use meower_shared::JwtClaims;

use axum::extract::{ Extension, Path, State };
use axum::http::Method;
use axum::response::IntoResponse;
use base64::prelude::*;
use reqwest::{ Client, header, Method as ReqMethod };


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------
pub(crate) async fn handler
(
    State(api_url): State<String>,
    Path(path): Path<String>,
    Extension(jwt_claims): Extension<JwtClaims>,
    method: Method,
    body: String,
) -> impl IntoResponse
{
    let client = Client::new();
    let method = ReqMethod::from_bytes(method.to_string().as_bytes()).unwrap();
    let uri = format!("{}/{}", api_url, path);
    let jwt_claims = BASE64_STANDARD.encode
    (
        serde_json::to_string(&jwt_claims).unwrap()
    );

    let res = client
        .request(method, uri)
        .header(header::CONTENT_TYPE, "application/json")
        .bearer_auth(jwt_claims)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    res
}
