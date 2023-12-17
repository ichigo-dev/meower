//------------------------------------------------------------------------------
//! Mypage API.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::api::API_URL;
//use meower_entity::user_account_profile::Model as UserAccountProfileModel;


//------------------------------------------------------------------------------
/// Gets profile.
//------------------------------------------------------------------------------
pub async fn get_profile( app_state: &AppState ) -> String//UserAccountProfileModel
{
    let client = app_state.client();

    let url = API_URL.to_string() + "/mypage/get_profile";
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    //let profile: UserAccountProfileModel = serde_json::from_str(&body).unwrap();
    body
}
