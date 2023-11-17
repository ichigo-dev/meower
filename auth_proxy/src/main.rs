//------------------------------------------------------------------------------
//! Reverse proxy server for authentication for Meower app.
//!
//! If the user is already logged in, the request is proxied to the server that
//! responds with Meower's SPA, otherwise the user is redirected to the login
//! page.
//------------------------------------------------------------------------------

mod auth;
mod login;
mod assets;
mod proxy;

use std::net::SocketAddr;
use std::env;

use axum::Router;
use axum::routing::get;
use axum::body::Body;
use hyper::client::HttpConnector;

pub(crate) type Client = hyper::client::Client<HttpConnector, Body>;


//------------------------------------------------------------------------------
/// Main entry point.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    // Creates the application.
    let client = Client::new();
    let app = Router::new()
        .route("/login", get(login::handler))
        .route("/_assets/*path", get(assets::handler))
        .fallback(proxy::handler)
        .with_state(client);

    // Runs the server.
    let port = env::var("AUTH_PROXY_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
