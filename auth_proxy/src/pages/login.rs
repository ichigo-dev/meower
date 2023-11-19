//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use crate::auth;
use crate::Client;

use std::env;

use askama::Template;
use axum::response::{ Redirect, Response, IntoResponse };
use axum::http::StatusCode;
use axum::body::Body;
use axum::extract::{ State, Form };
use serde::Deserialize;


//------------------------------------------------------------------------------
/// Login page template.
//------------------------------------------------------------------------------
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}


//------------------------------------------------------------------------------
/// Handles login page.
//------------------------------------------------------------------------------
pub(crate) async fn get_handler() -> impl IntoResponse
{
    let template = LoginTemplate {};
    let body = template.render().unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))
        .unwrap()
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug)]
pub(crate) struct LoginForm
{
    email: String,
    password: String,
}


//------------------------------------------------------------------------------
/// Handler for login form.
//------------------------------------------------------------------------------
pub(crate) async fn post_handler
(
    State(client): State<Client>,
    Form(input): Form<LoginForm>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    // Try to login.
    if auth::login(&input.email, &input.password).await == false
    {
        return Err(Redirect::to("/login"));
    }

    // Makes JWT token.
    let jwt = auth::make_jwt("secret", "1");

    // Proxies to the frontend.
    let proxy_uri = env::var("PROXY_URL")
        .unwrap_or("http://frontend:9000".to_string())
        .parse()
        .unwrap();
    let mut response = client.get(proxy_uri).await.unwrap();
    response
        .headers_mut()
        .insert("Set-Cookie", format!("jwt={}", jwt).parse().unwrap());
    Ok(response)
}
