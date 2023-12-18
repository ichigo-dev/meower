//------------------------------------------------------------------------------
//! Profile APIs.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth, I18n };
use meower_api_schema::ApiResponse;
use meower_api_schema::mypage::profile::GetProfileResponse;
use meower_entity::user_account::Entity as UserAccount;
use meower_entity::user_account_profile::Entity as UserAccountProfile;
use meower_entity::user_account_profile::Model as UserAccountProfileModel;

use axum::Extension;
use axum::response::Json;
use axum::extract::State;
use sea_orm::*;

//------------------------------------------------------------------------------
/// Gets profile.
//------------------------------------------------------------------------------
pub async fn get_profile
(
    State(state): State<AppState>,
    Extension(auth): Extension<Auth>,
    Extension(i18n): Extension<I18n>,
) -> Json<ApiResponse<GetProfileResponse>>
{
    let hdb = state.hdb();
    let user_account_profile = match get_profile_inner(hdb, &auth, &i18n).await
    {
        Ok(user_account_profile) => user_account_profile,
        Err(e) => return Json(ApiResponse::ng(e)),
    };

    let data = GetProfileResponse
    {
        name: user_account_profile.name.unwrap_or("".to_string()),
    };
    Json(ApiResponse::ok(data))
}

async fn get_profile_inner<C>
(
    hdb: &C,
    auth: &Auth,
    i18n: &I18n,
) -> Result<UserAccountProfileModel, String>
where
    C: ConnectionTrait,
{
    let claims = auth.claims();

    // Finds the user account.
    let user_account = match UserAccount::find_by_user_account_name(&claims.uan)
        .one(hdb)
        .await
        .unwrap()
    {
        Some(user_account) => user_account,
        None => return Err(i18n.get("mypage.profile.api.error.user_account.not_found")),
    };

    // Finds the user account profile.
    let user_account_profile = match user_account
        .find_related(UserAccountProfile)
        .one(hdb)
        .await
        .unwrap()
    {
        Some(user_account_profile) => user_account_profile,
        None => return Err(i18n.get("mypage.profile.api.error.user_account_profile.not_found")),
    };

    Ok(user_account_profile)
}
