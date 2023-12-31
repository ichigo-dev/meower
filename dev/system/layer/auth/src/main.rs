//------------------------------------------------------------------------------
//! Authentication.
//------------------------------------------------------------------------------

mod config;
mod state;
mod layers;
mod pages;
mod utils;

pub(crate) use config::Config;
pub(crate) use state::AppState;

use axum::{ Router, middleware };
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

    // Creates the authentication routes.
    let auth_routes = Router::new()
        .route("/login", get(pages::login::get_handler))
        .route("/login", post(pages::login::post_handler))
        .route("/signup", get(pages::signup::get_handler))
        .route("/signup", post(pages::signup::post_handler))
        .route("/verify_code", post(pages::verify_code::post_handler))
        .route
        (
            "/resend_verify_code",
            get(pages::resend_verify_code::get_handler),
        )
        .route
        (
            "/resend_verify_code",
            post(pages::resend_verify_code::post_handler),
        )
        .route("/forgot_password", get(pages::forgot_password::get_handler))
        .route("/forgot_password", post(pages::forgot_password::post_handler))
        .route
        (
            "/reset_password/:token",
            get(pages::reset_password::get_handler)
        )
        .route
        (
            "/reset_password/:token",
            post(pages::reset_password::post_handler)
        )
        .layer
        (
            middleware::from_fn_with_state
            (
                state.clone(),
                layers::protect_auth::layer,
            )
        )
        .nest_service("/assets", ServeDir::new("assets"));

    // Creates the application routes.
    let routes = Router::new()
        .fallback(utils::proxy::handler)
        .layer
        (
            middleware::from_fn_with_state
            (
                state.clone(),
                layers::protect::layer,
            )
        )
        .nest("/auth", auth_routes)
        .layer
        (
            middleware::from_fn_with_state(state.clone(), layers::i18n::layer)
        )
        .with_state(state);

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
