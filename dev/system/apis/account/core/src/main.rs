//------------------------------------------------------------------------------
//! Account API.
//------------------------------------------------------------------------------

mod config;
mod graphql;
mod state;

pub(crate) use config::Config;
pub(crate) use state::AppState;

use std::fs::File;
use std::io::Write;

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

    // Exports the GraphQL schema.
    File::create("schema.graphql")
        .unwrap()
        .write_all(state.schema.sdl().as_bytes())
        .unwrap();

    // Creates the application routes.
    let routes = Router::new()
        .route("/graphql", post(graphql::handler))
        .with_state(state.clone());

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", state.config.port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
