//------------------------------------------------------------------------------
//! Authentication server.
//------------------------------------------------------------------------------

mod config;
mod handlers;
mod layers;
mod state;
mod utils;

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
    let state = AppState::init().await;

    let auth_routes = Router::new()
        .route("/login", get(handlers::login::get_handler))
        .route("/login", post(handlers::login::post_handler))
        .route("/signup", get(handlers::signup::get_handler))
        .route("/signup", post(handlers::signup::post_handler))
        .route("/verify_code", post(handlers::verify_code::post_handler))
        .route
        (
            "/resend_verify_code",
            get(handlers::resend_verify_code::get_handler)
        )
        .route
        (
            "/resend_verify_code",
            post(handlers::resend_verify_code::post_handler)
        )
        .route("/forgot_password", get(handlers::forgot_password::get_handler))
        .route("/forgot_password", post(handlers::forgot_password::post_handler))
        .route
        (
            "/reset_password/:token",
            get(handlers::reset_password::get_handler)
        )
        .route
        (
            "/reset_password/:token",
            post(handlers::reset_password::post_handler)
        )
        .route
        (
            "/request_token/:code",
            get(handlers::request_token::get_handler)
        );

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
        .with_state(state.clone());

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", state.config.port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
