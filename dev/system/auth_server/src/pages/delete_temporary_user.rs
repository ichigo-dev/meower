//------------------------------------------------------------------------------
//! Delete temporary user.
//------------------------------------------------------------------------------

use crate::{ AppState, I18n };
use crate::pages::forgot_password::ForgotPasswordTemplate;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, IntoResponse };
use axum::extract::{ State, Path };
use sea_orm::prelude::*;
use sea_orm::TransactionTrait;


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Path(token): Path<String>,
) -> impl IntoResponse
{
    let hdb = state.hdb();

    // Deletes temporary user.
    let tsx = hdb.begin().await.unwrap();
    if let Err(e) = delete_temporary_user(&tsx, &i18n, &token).await
    {
        tsx.rollback().await.unwrap();
        let template = ForgotPasswordTemplate
        {
            i18n,
            errors: vec![e],
            ..Default::default()
        };
        return Html(template.render().unwrap());
    }

    tsx.commit().await.unwrap();
    let template = ForgotPasswordTemplate
    {
        i18n,
        deleted_temporary_user: true,
        ..Default::default()
    };
    Html(template.render().unwrap())
}


//------------------------------------------------------------------------------
/// Deletes temporary user.
//------------------------------------------------------------------------------
async fn delete_temporary_user<C>
(
    hdb: &C,
    i18n: &I18n,
    token: &str,
) -> Result<(), String>
where
    C: ConnectionTrait,
{
    let temporary_user = match TemporaryUserEntity::find_by_token(token)
        .one(hdb)
        .await
        .unwrap()
    {
        Some(temporary_user) => temporary_user,
        None =>
        {
            return Err(i18n.get(""));
        }
    };

    if let Err(e) = temporary_user.delete(hdb).await
    {
        return Err(e.to_string());
    }

    Ok(())
}
