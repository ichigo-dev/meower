//------------------------------------------------------------------------------
//! Login callback handler.
//------------------------------------------------------------------------------

use axum::response::{ Redirect, IntoResponse };
use axum::http::StatusCode;
use axum::extract::Query;
use hyper::Request;
use hyper_util::client::legacy::{ connect::HttpConnector, Client };
use serde::Deserialize;


//------------------------------------------------------------------------------
/// Response.
//------------------------------------------------------------------------------
#[derive(Deserialize)]
pub struct Parameter
{
    code: String,
}


//------------------------------------------------------------------------------
/// Handle.
//------------------------------------------------------------------------------
pub async fn handler
(
    query: Option<Query<Parameter>>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let code = match query
    {
        Some(query) => query.code.clone(),
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let client = Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(HttpConnector::new());
    let req = Request::builder()
        .uri("")
        .body(axum::body::Body::empty())
        .unwrap();
    let res = client.request(req).await.unwrap();

    Ok(StatusCode::OK)
}
