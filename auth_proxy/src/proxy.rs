//------------------------------------------------------------------------------
//! Proxies requests to the frontend.
//------------------------------------------------------------------------------

use crate::auth;
use crate::Client;

use std::env;

use axum::http::Request;
use axum::response::{ Redirect, IntoResponse };
use axum::body::Body;
use axum::extract::State;
use hyper::Uri;


//------------------------------------------------------------------------------
/// Proxies requests to the frontend.
//------------------------------------------------------------------------------
pub(crate) async fn handler
(
    State(client): State<Client>,
    mut req: Request<Body>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    // If the user is not logged in, redirects to the login page.
    if auth::is_logined().await == false
    {
        return Err(Redirect::to("/login"));
    }

    // Proxies the request to the frontend.
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let proxy_uri = env::var("PROXY_URL")
        .unwrap_or("http://frontend:9000".to_string());
    let uri = format!("{}{}", proxy_uri, path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client.request(req).await.unwrap())
}
