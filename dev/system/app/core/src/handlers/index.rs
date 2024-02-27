//------------------------------------------------------------------------------
//! Index page.
//------------------------------------------------------------------------------

use crate::AppState;

use meower_app_entity::user_token::Column as UserTokenColumn;
use meower_app_entity::user_token::Entity as UserTokenEntity;

use std::fs::File;
use std::io::Read;

use axum::extract::{ Extension, State };
use axum::http::StatusCode;
use axum::response::{ Html, IntoResponse };
use html_editor::{ parse, Node };
use html_editor::operation::*;
use sea_orm::{ ColumnTrait, EntityTrait, QueryFilter };


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn handler
(
    State(state): State<AppState>,
    Extension(user_token): Extension<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = &state.config;

    // Finds the user token.
    let user_token = match UserTokenEntity::find()
        .filter(UserTokenColumn::Token.eq(user_token))
        .one(&state.hdb)
        .await
        .unwrap()
    {
        Some(user_token) => user_token,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    // Loads the index.html file.
    let mut index_html = String::new();
    let mut index_html_file = File::open("public/index.html").unwrap();
    index_html_file.read_to_string(&mut index_html).unwrap();

    let mut dom = parse(&index_html).unwrap();
    let elm_values = Node::new_element("div", vec![], vec!
    [
        Node::new_element("span", vec!
        [
            ("id", &config.client_id_key),
            ("data-value", &config.client_id),
        ], vec![]),
        Node::new_element("span", vec!
        [
            ("id", &config.access_token_key),
            ("data-value", &user_token.access_token),
        ], vec![]),
        Node::new_element("span", vec!
        [
            ("id", &config.public_user_id_key),
            ("data-value", &user_token.public_user_id),
        ], vec![]),
        Node::new_element("span", vec!
        [
            ("id", &config.user_email_key),
            ("data-value", &user_token.user_email),
        ], vec![]),
    ]);
    let html = dom
        .insert_to(&Selector::from("body"), elm_values)
        .trim()
        .html();

    Ok(Html(html))
}
