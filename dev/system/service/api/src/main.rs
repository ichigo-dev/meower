//------------------------------------------------------------------------------
//! API server.
//------------------------------------------------------------------------------

mod config;
mod state;
mod apis;
mod layers;

pub(crate) use config::Config;
pub(crate) use state::AppState;

use meower_layer::ProtectedLayer;

use axum::{ Router, middleware };
use axum::routing::get;
use tokio::net::TcpListener;

// Loads the locales.
rust_i18n::i18n!("locales");


//------------------------------------------------------------------------------
/// Main entry point.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    // Initializes the configuration and state.
    let config = Config::init();
    let port = config.port;
    let jwt_audience = config.jwt_audience.clone();
    let jwt_secret = config.jwt_secret.clone();
    let auth_server_url = config.auth_server_url.clone();
    let state = AppState::init(config).await;

    // Creates the mypage api routes.
    let mypage_routes = Router::new()
        .route("/get_profile", get(apis::mypage::get_profile::get_handler));

    let api_routes = Router::new()
        .nest("/mypage", mypage_routes);

    // Creates the application routes.
    let routes = Router::new()
        .nest("/api", api_routes)
        .layer
        (
            middleware::from_fn_with_state(state.clone(), layers::i18n::layer)
        )
        .layer
        (
            ProtectedLayer::new(&jwt_audience, &jwt_secret, &auth_server_url)
        )
        .with_state(state);

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
