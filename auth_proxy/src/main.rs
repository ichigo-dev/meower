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

use axum::Router;
use axum::routing::{ get, post };
use axum::body::Body;
use hyper::client::HttpConnector;

pub(crate) type Client = hyper::client::Client<HttpConnector, Body>;


//------------------------------------------------------------------------------
/// Application state.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct AppState
{
    client: Client,
    config: Config,
    auth: Auth,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Creates a new application state.
    //--------------------------------------------------------------------------
    pub(crate) fn new( client: Client, config: Config, auth: Auth ) -> Self
    {
        Self { client, config, auth }
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

    //--------------------------------------------------------------------------
    /// Returns the auth.
    //--------------------------------------------------------------------------
    pub(crate) fn auth( &self ) -> &Auth
    {
        &self.auth
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
    let auth = Auth::init();
    let app_state = AppState::new(client, config.clone(), auth);

    // Creates the application.
    let app = Router::new()
        .route("/login", get(login::get_handler))
        .route("/login", post(login::post_handler))
        .route("/_assets/*path", get(assets::handler))
        .fallback(proxy::handler)
        .with_state(app_state);

    // Runs the server.
    let port = config.port();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
