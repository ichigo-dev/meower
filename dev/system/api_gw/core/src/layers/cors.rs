//------------------------------------------------------------------------------
//! CORS layer.
//------------------------------------------------------------------------------

use crate::AppState;

use axum::body::Body;
use axum::extract::State;
use axum::http::header;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;


//------------------------------------------------------------------------------
/// Layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    state: State<AppState>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse
{
    let cors_origins = state.cors_origins.read().await;

    let mut res = next.run(req).await;
    res.headers_mut().insert
    (
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        cors_origins.join(",").parse().unwrap(),
    );
    res.headers_mut().insert
    (
        header::ACCESS_CONTROL_ALLOW_METHODS,
        ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
            .join(",")
            .parse()
            .unwrap(),
    );
    res.headers_mut().insert
    (
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        ["Authorization", "Content-Type", &config.client_id_key]
            .join(",")
            .parse()
            .unwrap(),
    );
    println!("res: {:#?}", res);
    res
}
