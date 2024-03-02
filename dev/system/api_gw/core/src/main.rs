//------------------------------------------------------------------------------
//! Authentication server.
//------------------------------------------------------------------------------

mod config;
mod forward;
mod layers;
mod state;

pub(crate) use config::Config;
pub(crate) use state::AppState;

use axum::{ Router, middleware };
use axum::extract::DefaultBodyLimit;
use axum::http::{ header, Method };
use axum::routing::any;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

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
    let routes = Router::new()
        .route
        (
            "/account/*path",
            any(forward::handler)
                .with_state(state.config.account_api_url.clone())
        )
        .layer
        (
            middleware::from_fn_with_state
            (
                state.clone(),
                layers::protected::layer
            )
        )
        .layer
        (
            CorsLayer::new()
                .allow_methods(
                [
                    Method::GET,
                    Method::POST,
                    Method::OPTIONS,
                    Method::PUT,
                    Method::DELETE,
                ])
                .allow_headers(
                [
                    header::AUTHORIZATION,
                    header::CONTENT_TYPE,
                    state.config.client_id_key.clone().parse().unwrap(),
                ])
        )
        .layer
        (
            middleware::from_fn_with_state
            (
                state.clone(),
                layers::cors_ext::layer
            )
        )
        .layer(DefaultBodyLimit::max(BODY_LIMIT))
        .with_state(state.clone());

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", state.config.port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
