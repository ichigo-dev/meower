//------------------------------------------------------------------------------
//! Authentication server.
//------------------------------------------------------------------------------

mod config;
mod handlers;
mod layers;
mod state;

pub(crate) use config::Config;
pub(crate) use state::AppState;

use axum::{ Router, middleware };
use axum::routing::get;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;


//------------------------------------------------------------------------------
/// Main entry point.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    // Creates the application routes.
    let state = AppState::init().await;
    let routes = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .layer
        (
            middleware::from_fn_with_state
            (
                state.clone(),
                layers::protected::layer
            )
        )
        .route("/auth/callback", get(handlers::auth_callback::get_handler))
        .with_state(state.clone());

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", state.config.port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
