//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use askama::Template;
use axum::response::Response;
use axum::http::StatusCode;
use axum::body::Body;


//------------------------------------------------------------------------------
/// Template.
//------------------------------------------------------------------------------
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}


//------------------------------------------------------------------------------
/// Handler.
//------------------------------------------------------------------------------
pub async fn handler() -> Response<Body>
{
    let template = LoginTemplate {};
    let body = template.render().unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))
        .unwrap()
}
