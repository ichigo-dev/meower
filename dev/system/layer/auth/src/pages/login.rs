//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use meower_entity::traits::validate::ValidateExt;
use meower_entity::user::Entity as UserEntity;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_entity::user_jwt_subject::ActiveModel as ActiveUserJwtSubject;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use rust_i18n::t;
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::{ ActiveValue, TransactionTrait };


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "login.html")]
struct LoginTemplate
{
    input: LoginForm,
    errors: Vec<String>,
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug, Default)]
pub(crate) struct LoginForm
{
    email: String,
    password: String,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler()-> impl IntoResponse
{
    let template = LoginTemplate::default();
    Html(template.render().unwrap())
}
