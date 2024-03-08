//------------------------------------------------------------------------------
//! Avatar handler.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_account_entity::account_profile_avatar::Column as AccountProfileAvatarColumn;
use meower_account_entity::account_profile_avatar::Entity as AccountProfileAvatarEntity;

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

    let (avatar_file, mime_type) = match AccountProfileAvatarEntity::find()
        .filter(AccountProfileAvatarColumn::FileKey.eq(&file_key))
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(avatar_entity) =>
        {
            let avatar_path = StoragePath::from
            (
                format!("{}/{}", config.avatar_path, file_key)
            );
            match storage.get(&avatar_path).await
            {
                Ok(avatar) =>
                {
                    (avatar, avatar_entity.content_type.clone())
                },
                Err(_) =>
                {
                    let default_path = StoragePath::from
                    (
                        config.default_avatar_path.clone()
                    );
                    match storage.get(&default_path).await
                    {
                        Ok(default) =>
                        {
                            (default, config.default_avatar_mime.clone())
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
                config.default_avatar_path.clone()
            );
            match storage.get(&default_path).await
            {
                Ok(default) =>
                {
                    (default, config.default_avatar_mime.clone())
                },
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        },
    };

    let headers = [(header::CONTENT_TYPE, mime_type)];
    let bytes = avatar_file.bytes().await.unwrap();
    Ok((headers, bytes).into_response())
}
