//------------------------------------------------------------------------------
//! Protected layer.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::pages::bad_request::PageTemplate;

use meower_auth_entity::client_application::Column as ClientApplicationColumn;
use meower_auth_entity::client_application::Entity as ClientApplicationEntity;

use askama::Template;
use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{ Html, IntoResponse };
use sea_orm::{ ColumnTrait, EntityTrait, QueryFilter };


//------------------------------------------------------------------------------
/// Layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = state.config;
    let hdb = state.hdb;

    let client_id = match req.headers().get(config.client_id_key.as_str())
    {
        Some(client_id) => client_id.to_str().unwrap_or("").to_string(),
        None =>
        {
            return Err(Html(PageTemplate::default().render().unwrap()));
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

    req.extensions_mut().insert(client_application);
    Ok(next.run(req).await)
}
