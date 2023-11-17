use crate::auth;
use crate::Client;

use std::env;

use axum::http::Request;
use axum::response::Response;
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
) -> Response<Body>
{
    if auth::is_logined().await
    {
        return crate::login::handler().await;
    }

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let proxy_url = env::var("PROXY_URL")
        .unwrap_or("http://frontend:9000".to_string());
    let uri = format!("{}{}", proxy_url, path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    client.request(req).await.unwrap()
}
