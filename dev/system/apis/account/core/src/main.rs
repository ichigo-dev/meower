//------------------------------------------------------------------------------
//! Account API.
//------------------------------------------------------------------------------

mod config;
mod graphql;
mod state;

pub(crate) use config::Config;
use graphql::{ QueryRoot, MutationRoot };
pub(crate) use state::AppState;

use async_graphql::{
    EmptySubscription,
    Schema,
};
use async_graphql_axum::GraphQL;
use axum::Router;
use axum::routing::post_service;
use tokio::net::TcpListener;


//------------------------------------------------------------------------------
/// Main entry point.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    // Creates the application routes.
    let state = AppState::init().await;
    let schema = Schema::build
        (
            QueryRoot::default(),
            MutationRoot,
            EmptySubscription
        )
        .data(state.clone())
        .finish();

    let routes = Router::new()
        .route("/graphql", post_service(GraphQL::new(schema)))
        .with_state(state.clone());

    // Starts the server.
    let listener = TcpListener::bind(format!("0.0.0.0:{}", state.config.port))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}
