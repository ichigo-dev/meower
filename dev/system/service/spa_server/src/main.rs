//------------------------------------------------------------------------------
//! SPA delivery server.
//------------------------------------------------------------------------------

mod config;

pub(crate) use config::Config;

use meower_layer::ProtectedLayer;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::{ ServeDir, ServeFile };


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

    // Creates the application routes.
    let routes = Router::new()
        .nest_service
        (
            "/",
            ServeDir::new("public")
                .fallback(ServeFile::new("public/index.html"))
        )
        .layer
        (
            ProtectedLayer::new
            (
                &jwt_audience,
                &jwt_secret,
                &auth_server_url,
                false,
            )
        );

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
