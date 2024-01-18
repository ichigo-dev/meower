//------------------------------------------------------------------------------
//! Common routes.
//------------------------------------------------------------------------------

mod handlers;

use handlers::*;

use axum::Router;
use axum::routing::get;
use reqwest::Client;


pub fn get_auth_callbacks_route() -> Router
{
    let client = Client::new();
    let routes = Router::new()
        .route("/login_callback", get(login_callback::handler))
        .with_state(client);
    routes
}
