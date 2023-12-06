//------------------------------------------------------------------------------
//! Authentication server for Meower app.
//!
//! If the user is already logged in, the request is proxied to the server that
//! responds with Meower's SPA, otherwise the user is redirected to the login
//! page.
//------------------------------------------------------------------------------

mod layers;
mod pages;
mod assets;
mod proxy;

use meower_core::{ Auth, Config, I18n };
use layers::{ auth, i18n };
use pages::{ login, signup };

use std::env;
use std::net::SocketAddr;

use axum::{ Router, middleware };
use axum::body::Body;
use axum::routing::{ get, post };
use hyper::client::HttpConnector;
use sea_orm::{ Database, DbConn };

pub(crate) type Client = hyper::client::Client<HttpConnector, Body>;


//------------------------------------------------------------------------------
/// Application state.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct AppState
{
    hdb: DbConn,
    client: Client,
    config: Config,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Creates a new application state.
    //--------------------------------------------------------------------------
    pub(crate) fn new( hdb: DbConn, client: Client, config: Config ) -> Self
    {
        Self { hdb, client, config }
    }

    //--------------------------------------------------------------------------
    /// Returns the db handler.
    //--------------------------------------------------------------------------
    pub(crate) fn hdb( &self ) -> &DbConn
    {
        &self.hdb
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
    let hdb = Database::connect(config.database_url().to_string())
        .await
        .expect("Failed to setup the database");
    let app_state = AppState::new(hdb, client, config.clone());

    // Creates the application.
    let app = Router::new()
        .route("/login", get(login::get_handler))
        .route("/login", post(login::post_handler))
        .route("/signup", get(signup::get_handler))
        .route("/signup", post(signup::post_handler))
        .route("/_assets/*path", get(assets::handler))
        .fallback(proxy::handler)
        .layer(middleware::from_fn_with_state(app_state.clone(), i18n::layer))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth::layer))
        .with_state(app_state);

    // Runs the server.
    let port = env::var("AUTH_SERVER_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
