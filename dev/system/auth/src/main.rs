//------------------------------------------------------------------------------
//! Authentication.
//------------------------------------------------------------------------------

mod config;
mod state;

use axum::{ Router, middleware };
use axum::routing::{ get, post };

pub(crate) use config::Config;
pub(crate) use state::State;

use tokio::net::TcpListener;


//------------------------------------------------------------------------------
/// Main entry point.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    // Initializes the configuration and state.
    let config = Config::init();
    let port = config.port();
    let state = State::init(config).await;

    // Creates the authentication routes.
    let auth_routes = Router::new();

    // Creates the application routes.
    let routes = Router::new()
        .route("/auth", auth_routes)
        .with_state(state);

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
