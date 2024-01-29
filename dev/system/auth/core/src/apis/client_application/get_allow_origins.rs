//------------------------------------------------------------------------------
//! Gets the origins.
//------------------------------------------------------------------------------

use crate::AppState;

use meower_auth_entity::client_application_allow_origin::Entity as ClientApplicationAllowOriginEntity;

use axum::extract::{ Request, State };
use axum::http::StatusCode;
use axum::response::{ IntoResponse, Json };
use sea_orm::EntityTrait;


//------------------------------------------------------------------------------
/// Response.
//------------------------------------------------------------------------------
#[derive(serde::Serialize)]
pub(crate) struct Response
{
    origins: Vec<String>,
}


//------------------------------------------------------------------------------
/// Gets the origins.
//------------------------------------------------------------------------------
pub(crate) async fn get_handler
(
    state: State<AppState>,
    req: Request,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = &state.config;

    let api_key = req.headers()
        .get(&config.api_key_key)
        .map(|value| value.to_str().unwrap().to_string())
        .unwrap_or(String::new());
    if api_key != config.api_key_val
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let origins = ClientApplicationAllowOriginEntity::find()
        .all(&state.hdb)
        .await
        .unwrap_or(Vec::new())
        .into_iter()
        .map(|x| x.allow_origin)
        .collect();

    let res = Response { origins };

    Ok(Json(res))
}
