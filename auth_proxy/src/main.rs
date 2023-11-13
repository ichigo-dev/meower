//------------------------------------------------------------------------------
/// Authentication server.
//------------------------------------------------------------------------------

use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main()
{
    let app = Router::new()
        .route("/", get(handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9001));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn handler() -> &'static str
{
    "Hello, World!"
}
