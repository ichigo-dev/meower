//------------------------------------------------------------------------------
//! Authentication server.
//------------------------------------------------------------------------------

mod config;
mod layers;
mod state;

pub(crate) use config::Config;
pub(crate) use state::AppState;

use axum::{ Router, middleware };
use axum::response::Redirect;
use axum::routing::{ get, post };
use tokio::net::TcpListener;


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
//        .nest("/", get(hoge))
//        .layer
//        (
//            middleware::from_fn_with_state
//            (
//                state.clone(),
//                layers::protected::layer
//            )
//        )
        .with_state(state);

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
