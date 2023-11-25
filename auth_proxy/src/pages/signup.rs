//------------------------------------------------------------------------------
//! Signup page.
//------------------------------------------------------------------------------

use meower_entity::user::{ self, Entity as User, ActiveModel as ActiveUser };
use meower_utility::Validator;
use crate::{ AppState, Auth };

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Redirect, IntoResponse };
use axum::extract::{ State, Form };
use serde::Deserialize;
use sea_orm::{
    ColumnTrait,
    EntityTrait,
    QueryFilter,
    ActiveValue,
    ActiveModelTrait,
};
use sea_query::Condition;


//------------------------------------------------------------------------------
/// Signup page template.
//------------------------------------------------------------------------------
#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate
{
    errors: Vec<String>,
}

impl Default for SignupTemplate
{
    fn default() -> Self
    {
        Self { errors: Vec::new() }
    }
}


//------------------------------------------------------------------------------
/// Handles signup page.
//------------------------------------------------------------------------------
pub(crate) async fn get_handler
(
    Extension(auth): Extension<Auth>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    if auth.is_logined().await
    {
        return Err(Redirect::to("/"));
    }

    let template = SignupTemplate::default();
    Ok(Html(template.render().unwrap()))
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug)]
pub(crate) struct SignupForm
{
    account_name: String,
    email: String,
    email_confirm: String,
    password: String,
    password_confirm: String,
}


//------------------------------------------------------------------------------
/// Handler for signup form.
//------------------------------------------------------------------------------
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Form(input): Form<SignupForm>,
) -> impl IntoResponse
{
    let hdb = state.hdb();
    let config = state.config();

    // Validates account name.
    let exists_user = User::find()
        .filter(user::Column::AccountName.contains(&input.account_name))
        .one(hdb)
        .await
        .unwrap();
    let mut account_name_validator = Validator::new(&input.account_name)
        .not_empty("Account name is empty.")
        .min_len(3, "Account name is too short.")
        .max_len(255, "Account name is too long.")
        .regex(r"^[a-zA-Z0-9_]+$", "Account name must contain only letters, numbers, and underscores.")
        .custom(|_| { exists_user.is_none() }, "Account name already exists.")
        .validate();
    if account_name_validator.validate() == false
    {
        let errors = account_name_validator.errors();
        let template = SignupTemplate { errors: errors.to_vec() };
        return Html(template.render().unwrap());
    }

    // Validates email.
    let exsits_user = User::find()
        .filter(user::Column::Email.contains(&input.email))
        .one(hdb)
        .await
        .unwrap();
    let mut email_validator = Validator::new(&input.email)
        .not_empty("Email is empty.")
        .max_len(255, "Email is too long.")
        .is_email("Email is invalid.")
        .same(&input.email_confirm, "Emails do not match.")
        .custom(|_| { exsits_user.is_none() }, "Email already exists.")
        .validate();
    if email_validator.validate() == false
    {
        let errors = email_validator.errors();
        let template = SignupTemplate { errors: errors.to_vec() };
        return Html(template.render().unwrap());
    }

    // Validates password.
    let mut password_validator = Validator::new(&input.password)
        .not_empty("Password is empty.")
        .min_len(8, "Password is too short.")
        .max_len(255, "Password is too long.")
        .regex(r".*[a-zA-Z].*", "Password must contain at least one letter.")
        .regex(r".*[0-9].*", "Password must contain at least one number.")
        .regex(r".*[!@#$%^&*()].*", "Password must contain at least one special character.")
        .same(&input.password_confirm, "Passwords do not match.")
        .validate();
    if password_validator.validate() == false
    {
        let errors = password_validator.errors();
        let template = SignupTemplate { errors: errors.to_vec() };
        return Html(template.render().unwrap());
    }

    // Creates a new user.
    let user = ActiveUser
    {
        id: ActiveValue::NotSet,
        account_name: ActiveValue::Set(input.account_name),
        email: ActiveValue::Set(input.email),
        password: ActiveValue::Set(Auth::password_hash(&input.password, config)),
    };
    user.insert(hdb).await.unwrap();

    let template = SignupTemplate::default();
    return Html(template.render().unwrap());
}
