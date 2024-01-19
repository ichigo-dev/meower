//------------------------------------------------------------------------------
//! Authentication server.
//------------------------------------------------------------------------------

mod config;
mod layers;
mod pages;
mod state;

pub(crate) use config::Config;
pub(crate) use state::AppState;

use axum::{ Router, middleware };
use axum::response::Redirect;
use axum::routing::{ get, post };
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

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
    let state = AppState::init(config).await;

    let auth_routes = Router::new()
        .route("/login", get(pages::login::get_handler))
        .route("/login", post(pages::login::post_handler));

    // Creates the application routes.
    let routes = Router::new()
        .nest("/auth", auth_routes)
        .fallback(Redirect::temporary("/auth/login"))
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
            middleware::from_fn_with_state(state.clone(), layers::i18n::layer)
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state);

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
