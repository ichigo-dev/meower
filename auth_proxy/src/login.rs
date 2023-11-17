//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use askama::Template;
use axum::response::{ Response, IntoResponse };
use axum::http::StatusCode;
use axum::body::Body;


//------------------------------------------------------------------------------
/// Login page template.
//------------------------------------------------------------------------------
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}


//------------------------------------------------------------------------------
/// Handles login page.
//------------------------------------------------------------------------------
pub async fn handler() -> impl IntoResponse
{
    let template = LoginTemplate {};
    let body = template.render().unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))
        .unwrap()
}
