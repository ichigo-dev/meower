//------------------------------------------------------------------------------
//! Meower backend server.
//------------------------------------------------------------------------------

mod layers;
mod mypage;

use layers::*;
use meower_core::*;

use std::env;
use std::net::SocketAddr;

use axum::{ Router, middleware };
use axum::routing::get;
use sea_orm::{ Database, DbConn };


//------------------------------------------------------------------------------
/// Application state.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct AppState
{
    hdb: DbConn,
    config: Config,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Creates a new application state.
    //--------------------------------------------------------------------------
    pub(crate) fn new( hdb: DbConn, config: Config ) -> Self
    {
        Self { hdb, config }
    }

    //--------------------------------------------------------------------------
    /// Returns the db handler.
    //--------------------------------------------------------------------------
    pub(crate) fn hdb( &self ) -> &DbConn
    {
        &self.hdb
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
    let config = Config::new();
    let hdb = Database::connect(config.get("database.url"))
        .await
        .expect("Failed to setup the database");
    let app_state = AppState::new(hdb, config.clone());

    // Creates the application.
    let mypage_router = Router::new()
        .route("/get_profile", get(mypage::profile::get_profile));

    let app = Router::new()
        .nest("/mypage", mypage_router)
        .layer(middleware::from_fn_with_state(app_state.clone(), auth::layer))
        .layer(middleware::from_fn_with_state(app_state.clone(), i18n::layer))
        .with_state(app_state);

    // Runs the server.
    let port = env::var("BACKEND_PORT")
        .unwrap_or("9001".to_string())
        .parse()
        .unwrap_or(9001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
