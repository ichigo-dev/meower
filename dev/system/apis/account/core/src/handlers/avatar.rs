//------------------------------------------------------------------------------
//! Avatar handler.
//------------------------------------------------------------------------------

use crate::AppState;

use axum::extract::{ Path, State };
use axum::http::{ header, StatusCode };
use axum::response::IntoResponse;
use object_store::path::Path as StoragePath;


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub async fn get_handler
(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = &state.config;
    let storage = state.storage.clone();

    let default_path = StoragePath::from(config.default_avatar_path.clone());
    let default = match storage.get(&default_path).await
    {
        Ok(default) => default,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let headers = [(header::CONTENT_TYPE, &config.default_avatar_mime)];

    let bytes = default.bytes().await.unwrap();
    Ok((headers, bytes).into_response())
}
