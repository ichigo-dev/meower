//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use crate::{ AppState, JwtClaims, Auth, I18n, Config };
use meower_entity::Validate;
use meower_entity::user::Entity as UserEntity;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_entity::user_jwt_subject::ActiveModel as ActiveUserJwtSubject;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Response };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::{ ActiveValue, TransactionTrait };


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
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "login.html")]
struct LoginTemplate
{
    i18n: I18n,
    input: LoginForm,
    errors: Vec<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    Extension(i18n): Extension<I18n>,
) -> Html<String>
{
    let template = LoginTemplate
    {
        i18n,
        ..Default::default()
    };
    Html(template.render().unwrap())
}

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Form(input): Form<LoginForm>,
) -> Result<Response<Body>, Html<String>>
{
    let hdb = state.hdb();
    let config = state.config();

    let tsx = hdb.begin().await.unwrap();
    let jwt_cookie = match try_login(&tsx, &input, &i18n, &config).await
    {
        Ok(jwt_cookie) => jwt_cookie,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let template = LoginTemplate
            {
                i18n,
                input,
                errors: vec![e],
                ..Default::default()
            };
            return Err(Html(template.render().unwrap()));
        }
    };

    // Proxies to the frontend.
    tsx.commit().await.unwrap();
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, jwt_cookie.to_string())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}


//------------------------------------------------------------------------------
/// Tries to login.
//------------------------------------------------------------------------------
async fn try_login<C>
(
    hdb: &C,
    input: &LoginForm,
    i18n: &I18n,
    config: &Config,
) -> Result<String, String>
where
    C: ConnectionTrait,
{
    // Try to login.
    let user = match UserEntity::find_by_email(&input.email)
        .one(hdb)
        .await
        .unwrap()
    {
        Some(user) => user,
        None =>
        {
            let error = match TemporaryUserEntity::find_by_email(&input.email)
                .one(hdb)
                .await
                .unwrap()
            {
                Some(_) =>
                {
                    i18n.get("entrance.login.form.error.user_not_verified")
                },
                None =>
                {
                    i18n.get("entrance.login.form.error.user_not_found")
                },
            };
            return Err(error);
        }
    };

    if user.try_login(hdb, &input.password).await == false
    {
        return Err
        (
            i18n.get("entrance.login.form.error.invalid_password")
        );
    }

    // Creates a JWT subject.
    let user_jwt_subject = ActiveUserJwtSubject
    {
        user_id: ActiveValue::Set(user.user_id),
        ..Default::default()
    };
    let user_jwt_subject = match user_jwt_subject
        .validate_and_insert(hdb, &i18n)
        .await
    {
        Ok(subject) => subject,
        Err(e) => return Err(e.to_string()),
    };

    // Creates a JWT claims.
    let mut claims = JwtClaims::init_from_config(&config);
    claims.sub = user_jwt_subject.subject;

    match user.get_last_logined_user_account(hdb).await
    {
        Some(user_account) => claims.uan = user_account.user_account_name,
        None => {},
    }

    let auth = Auth::init(claims);
    Ok(auth.make_jwt_cookie(&config))
}
