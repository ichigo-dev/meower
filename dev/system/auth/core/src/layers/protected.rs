//------------------------------------------------------------------------------
//! Protected layer.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::pages::bad_request::PageTemplate;

use meower_auth_entity::client_application::Column as ClientApplicationColumn;
use meower_auth_entity::client_application::Entity as ClientApplicationEntity;

use askama::Template;
use axum::body::Body;
use axum::extract::{ Query, State };
use axum::http::{ header, Request };
use axum::middleware::Next;
use axum::response::{ Html, IntoResponse };
use axum_extra::extract::cookie::{ Cookie, CookieJar };
use sea_orm::{ ColumnTrait, EntityTrait, QueryFilter };
use serde::Deserialize;
use time::{ Duration, OffsetDateTime };

const COOKIE_EXPIERATION_MINUTES: i64 = 30;


//------------------------------------------------------------------------------
/// Params.
//------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub(crate) struct Params
{
    pub(crate) client_id: Option<String>,
}


//------------------------------------------------------------------------------
/// Layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    State(state): State<AppState>,
    Query(params): Query<Params>,
    cookie: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = state.config;
    let hdb = state.hdb;

    let client_id = match params.client_id
    {
        Some(client_id) => client_id,
        None =>
        {
            match cookie.get(&config.client_id_key)
                .map(|cookie| cookie.value().to_string())
            {
                Some(client_id) => client_id,
                None =>
                {
                    return Err(Html(PageTemplate::default().render().unwrap()));
                },
            }
        },
    };

    let client_application = match ClientApplicationEntity::find()
        .filter(ClientApplicationColumn::ClientId.eq(client_id))
        .one(&hdb)
        .await
        .unwrap()
    {
        Some(client_application) => client_application,
        None =>
        {
            return Err(Html(PageTemplate::default().render().unwrap()));
        },
    };

    let mut res = next.run(req).await;
    let expire = OffsetDateTime::now_utc()
        + Duration::minutes(COOKIE_EXPIERATION_MINUTES);
    res.headers_mut().insert
    (
        header::SET_COOKIE,
        Cookie::build((&config.client_id_key, &client_application.client_id))
            .path("/")
            .secure(true)
            .http_only(true)
            .expires(expire)
            .to_string()
            .parse()
            .unwrap()
    );
    Ok(res)
}
