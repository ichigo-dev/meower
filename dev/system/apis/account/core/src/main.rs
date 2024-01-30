//------------------------------------------------------------------------------
//! Account API.
//------------------------------------------------------------------------------

mod config;
mod graphql;
mod state;

pub(crate) use config::Config;
pub(crate) use state::AppState;

use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;


//------------------------------------------------------------------------------
/// Main entry point.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    // Creates the application routes.
    let state = AppState::init().await;
    let routes = Router::new()
        .route("/graphql", post(graphql::graphql_handler))
        .with_state(state.clone());

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", state.config.port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
