//------------------------------------------------------------------------------
//! SPA delivery server.
//------------------------------------------------------------------------------

mod config;
mod state;

pub(crate) use config::Config;
pub(crate) use state::AppState;

use axum::{ Router, middleware };
use tokio::net::TcpListener;
use tower_http::services::ServeDir;


//------------------------------------------------------------------------------
/// Main entry point.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    // Initializes the configuration and state.
    let config = Config::init();
    let port = config.port;
    let state = AppState::init(config).await;

    // Creates the application routes.
    let routes = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .with_state(state);

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
