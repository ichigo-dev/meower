//------------------------------------------------------------------------------
//! Reverse proxy server for authentication for Meower app.
//!
//! If the user is already logged in, the request is proxied to the server that
//! responds with Meower's SPA, otherwise the user is redirected to the login
//! page.
//------------------------------------------------------------------------------

mod config;
mod pages;
mod assets;
mod auth;
mod proxy;

use auth::Auth;
use config::Config;
use pages::login;

use std::net::SocketAddr;

use axum::{ Router, middleware };
use axum::body::Body;
use axum::routing::{ get, post };
use hyper::client::HttpConnector;

static JWT_COOKIE_KEY: &str = "token";

pub(crate) type Client = hyper::client::Client<HttpConnector, Body>;


//------------------------------------------------------------------------------
/// Application state.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct AppState
{
    client: Client,
    config: Config,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Creates a new application state.
    //--------------------------------------------------------------------------
    pub(crate) fn new( client: Client, config: Config ) -> Self
    {
        Self { client, config }
    }

    //--------------------------------------------------------------------------
    /// Returns the client.
    //--------------------------------------------------------------------------
    pub(crate) fn client( &self ) -> &Client
    {
        &self.client
    }

    //--------------------------------------------------------------------------
    /// Returns the config.
    //--------------------------------------------------------------------------
    pub(crate) fn config( &self ) -> &Config
    {
        &self.config
    }
}


//------------------------------------------------------------------------------
/// Main entry point.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    // Initializes the application state.
    let client = Client::new();
    let config = Config::init();
    let app_state = AppState::new(client, config.clone());

    // Creates the application.
    let app = Router::new()
        .route("/login", get(login::get_handler))
        .route("/login", post(login::post_handler))
        .route("/_assets/*path", get(assets::handler))
        .fallback(proxy::handler)
        .layer
        (
            middleware::from_fn_with_state
            (
                app_state.clone(),
                auth::auth_layer
            )
        )
        .with_state(app_state);

    // Runs the server.
    let port = config.port();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
