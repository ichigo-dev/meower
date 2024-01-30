//------------------------------------------------------------------------------
//! HTTP forward.
//------------------------------------------------------------------------------

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
    body: String,
) -> impl IntoResponse
{
    let client = Client::new();
    let method = ReqMethod::from_bytes(method.to_string().as_bytes()).unwrap();
    let uri = format!("{}/{}", api_url, path);
    println!("body: {}", body);

    let res = client
        .request(method, uri)
        .body(body)
//.body(r#"{"query":"\nquery Accounts {\n  accounts {\n    accountName\n  }\n}"}"#)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    res
}
