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

use meower_core::*;
use layers::*;
use pages::*;

use std::env;

use axum::{ Router, middleware };
use axum::response::Redirect;
use axum::body::Body;
use axum::routing::{ get, post };
use hyper_util::rt::TokioExecutor;
use hyper_util::client::legacy::connect::HttpConnector;
use sea_orm::{ Database, DbConn };
use tokio::net::TcpListener;

pub(crate) type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;


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
    let client = hyper_util::client::legacy::Client::<(), ()>::builder
    (
        TokioExecutor::new()
    )
        .build(HttpConnector::new());
    let config = Config::new();
    let hdb = Database::connect(config.get("database.url"))
        .await
        .expect("Failed to setup the database");
    let app_state = AppState::new(hdb, client, config.clone());

    // Creates the application.
    let redirect_login_page = get(|| async { Redirect::to("/auth/login") });
    let auth_routes = Router::new()
        .route("/login", get(login::get_handler))
        .route("/login", post(login::post_handler))
        .route("/signup", get(signup::get_handler))
        .route("/signup", post(signup::post_handler))
        .route("/verify_code", post(verify_code::post_handler))
        .route("/resend_verify_code", get(resend_verify_code::get_handler))
        .route("/resend_verify_code", post(resend_verify_code::post_handler))
        .route("/forgot_password", get(forgot_password::get_handler))
        .route("/forgot_password", post(forgot_password::post_handler))
        .route("/reset_password/:token", get(reset_password::get_handler))
        .route("/reset_password/:token", post(reset_password::post_handler))
        .route
        (
            "/delete_temporary_user/:token",
            get(delete_temporary_user::get_handler)
        )
        .route("/create_account/:sub", get(create_account::get_handler))
        .route("/create_account/:sub", post(create_account::post_handler))
        .fallback(redirect_login_page.clone());

    let app = Router::new()
        .nest("/auth", auth_routes)
        .route("/auth/", redirect_login_page.clone())
        .route("/_assets/*path", get(assets::handler))
        .fallback(proxy::handler)
        .layer(middleware::from_fn_with_state(app_state.clone(), auth::layer))
        .layer(middleware::from_fn_with_state(app_state.clone(), i18n::layer))
        .with_state(app_state);

    // Runs the server.
    let port = env::var("AUTH_SERVER_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap_or(8080);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
