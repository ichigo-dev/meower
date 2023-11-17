//------------------------------------------------------------------------------
//! Authentication server.
//------------------------------------------------------------------------------

mod login;

use std::net::SocketAddr;
use std::env;

use axum::response::Response;
use axum::http::Request;
use axum::body::Body;
use tower::make::Shared;


//------------------------------------------------------------------------------
/// Main entry point.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    let service = tower::service_fn(move |req: Request<_>|
    {
        async move { proxy(req).await }
    });

    // Run the server.
    let port = env::var("AUTH_PROXY_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(Shared::new(service))
        .await
        .unwrap();
}


//------------------------------------------------------------------------------
/// Proxies the request to the frontend.
//------------------------------------------------------------------------------
async fn proxy( req: Request<Body> ) -> Result<Response<Body>, hyper::Error>
{
    if true
    {
        return Ok(login::handler().await);
    }

    // Proxy the request to the frontend.
    let path = match req.uri().path_and_query()
    {
        Some(path) => path,
        None => return Ok(Response::new(Body::empty())),
    };
    let proxy_url = env::var("PROXY_URL")
        .unwrap_or("http://frontend:9000".to_string());
    let to_req = Request::builder()
        .uri(proxy_url + path.as_str())
        .method(req.method())
        .version(req.version())
        .body(req.into_body())
        .unwrap();

    let client = hyper::Client::new();
    client.request(to_req).await
}
