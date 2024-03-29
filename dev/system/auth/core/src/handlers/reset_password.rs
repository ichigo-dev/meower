//------------------------------------------------------------------------------
//! Reset password page. 
//------------------------------------------------------------------------------

use crate::AppState;
use crate::handlers::login::PageTemplate as LoginPageTemplate;
use crate::handlers::reset_password_success::PageTemplate as ResetPasswordSuccessPageTemplate;

use meower_auth_entity::user::Column as UserColumn;
use meower_auth_entity::user::Entity as UserEntity;
use meower_auth_entity::user_auth::Entity as UserAuthEntity;
use meower_auth_entity::user_auth::ActiveModel as UserAuthActiveModel;
use meower_auth_entity::reset_password_token::Column as ResetPasswordTokenColumn;
use meower_auth_entity::reset_password_token::Entity as ResetPasswordTokenEntity;
use meower_entity_ext::ValidateExt;

use askama::Template;
use axum::extract::{ Form, Path, State };
use axum::response::{ Html, IntoResponse };
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    EntityTrait,
    ModelTrait,
    QueryFilter,
    TransactionTrait,
};
use serde::Deserialize;


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "reset_password.html")]
pub(crate) struct PageTemplate
{
    pub(crate) input: FormData,
    pub(crate) input_error: FormError,
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------

// FormData
#[derive(Deserialize, Debug, Default)]
pub(crate) struct FormData
{
    pub(crate) password: String,
    pub(crate) password_confirm: String,
}

// Error
#[derive(Default)]
pub(crate) struct FormError
{
    pub(crate) password: Option<String>,
    pub(crate) password_confirm: Option<String>,
    pub(crate) other: Option<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    if ResetPasswordTokenEntity::find()
        .filter(ResetPasswordTokenColumn::Token.eq(token))
        .one(&state.hdb)
        .await
        .unwrap()
        .is_none()
    {
        let template = LoginPageTemplate::default();
        return Err(Html(template.render().unwrap()));
    }

    let template = PageTemplate::default();
    Ok(Html(template.render().unwrap()))
}

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Form(input): Form<FormData>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let tsx = state.hdb.begin().await.unwrap();

    // Finds a reset password token.
    let reset_password_token = match ResetPasswordTokenEntity::find()
            .filter(ResetPasswordTokenColumn::Token.eq(token))
            .one(&tsx)
            .await
            .unwrap()
    {
        Some(reset_password_token) => reset_password_token,
        None =>
        {
            tsx.rollback().await.unwrap();
            let template = LoginPageTemplate::default();
            return Err(Html(template.render().unwrap()));
        },
    };

    // Checks if the password matches the confirmation password
    if input.password != input.password_confirm
    {
        tsx.rollback().await.unwrap();
        let error = t!("pages.reset_password.form.password_confirm.error.not_match");
        let template = PageTemplate
        {
            input: input,
            input_error: FormError
            {
                password_confirm: Some(error),
                ..Default::default()
            },
        };
        return Err(Html(template.render().unwrap()));
    }

    // Finds a user.
    let user = match UserEntity::find()
        .filter(UserColumn::UserId.eq(reset_password_token.user_id))
        .one(&state.hdb)
        .await
        .unwrap()
    {
        Some(user) => user,
        None =>
        {
            tsx.rollback().await.unwrap();
            let template = LoginPageTemplate::default();
            return Err(Html(template.render().unwrap()));
        },
    };

    // Finds a user auth.
    let user_auth = match user
        .find_related(UserAuthEntity)
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(user_auth) => user_auth,
        None =>
        {
            tsx.rollback().await.unwrap();
            let template = LoginPageTemplate::default();
            return Err(Html(template.render().unwrap()));
        },
    };

    // Changes the password.
    let mut user_auth: UserAuthActiveModel = user_auth.into();
    user_auth.password = ActiveValue::set(input.password.clone());
    if let Err(e) = user_auth.validate_and_save(&tsx).await
    {
        let template = PageTemplate
        {
            input: input,
            input_error: FormError
            {
                other: Some(e.get_message()),
                ..Default::default()
            },
        };
        return Err(Html(template.render().unwrap()));
    };

    tsx.commit().await.unwrap();
    let template = ResetPasswordSuccessPageTemplate
    {
        ..Default::default()
    };
    Ok(Html(template.render().unwrap()))
}
