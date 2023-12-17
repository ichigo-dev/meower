//------------------------------------------------------------------------------
//! Mypage API.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_entity::user_account_profile::Model as UserAccountProfileModel;


//------------------------------------------------------------------------------
/// Gets profile.
//------------------------------------------------------------------------------
pub async fn get_profile( app_state: &AppState ) -> UserAccountProfileModel
{
    let config = app_state.config();
    let client = app_state.client();

    let url = config.get("system.backend_url") + "/mypage/get_profile";
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    let profile: UserAccountProfileModel = serde_json::from_str(&body).unwrap();
    profile
}
