//------------------------------------------------------------------------------
//! Cover handler.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_account_entity::account_profile_cover::Column as AccountProfileCoverColumn;
use meower_account_entity::account_profile_cover::Entity as AccountProfileCoverEntity;

use axum::extract::{ Path, State };
use axum::http::{ header, StatusCode };
use axum::response::IntoResponse;
use object_store::path::Path as StoragePath;
use sea_orm::{
    ColumnTrait,
    EntityTrait,
    QueryFilter,
    TransactionTrait,
};


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub async fn get_handler
(
    State(state): State<AppState>,
    Path(file_key): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let tsx = state.hdb.begin().await.unwrap();
    let config = &state.config;
    let storage = state.storage.clone();

    let (cover_file, mime_type) = match AccountProfileCoverEntity::find()
        .filter(AccountProfileCoverColumn::FileKey.eq(&file_key))
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(cover_entity) =>
        {
            let cover_path = StoragePath::from
            (
                format!("{}/{}", config.cover_path, file_key)
            );
            match storage.get(&cover_path).await
            {
                Ok(cover) =>
                {
                    (cover, cover_entity.content_type.clone())
                },
                Err(_) =>
                {
                    let default_path = StoragePath::from
                    (
                        config.default_cover_path
                    );
                    match storage.get(&default_path).await
                    {
                        Ok(default) =>
                        {
                            (default, config.default_cover_mime.to_string())
                        },
                        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                },
            }
        },
        None =>
        {
            let default_path = StoragePath::from
            (
                config.default_cover_path
            );
            match storage.get(&default_path).await
            {
                Ok(default) =>
                {
                    (default, config.default_cover_mime.to_string())
                },
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        },
    };

    let headers = [(header::CONTENT_TYPE, mime_type)];
    let bytes = cover_file.bytes().await.unwrap();
    Ok((headers, bytes).into_response())
}
