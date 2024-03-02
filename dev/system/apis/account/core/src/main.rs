//------------------------------------------------------------------------------
//! Account API.
//------------------------------------------------------------------------------

mod config;
mod graphql;
mod handlers;
mod layers;
mod state;

use handlers::*;
pub(crate) use config::Config;
pub(crate) use state::AppState;

use std::fs::File;
use std::io::Write;

use axum::{ Router, middleware };
use axum::extract::DefaultBodyLimit;
use axum::routing::{ get, post };
use tokio::net::TcpListener;

// Loads the locales.
rust_i18n::i18n!("locales");

// Body limit.
const BODY_LIMIT: usize = 30_000_000;


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
        .route("/avatar/:file_key", get(avatar::get_handler))
        .route("/cover/:file_key", get(cover::get_handler))
        .layer
        (
            middleware::from_fn_with_state(state.clone(), layers::i18n::layer)
        )
        .layer(DefaultBodyLimit::max(BODY_LIMIT))
        .with_state(state.clone());

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", state.config.port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
