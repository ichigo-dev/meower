//------------------------------------------------------------------------------
//! Mypage API.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::api::API_URL;


//------------------------------------------------------------------------------
/// Gets profile.
//------------------------------------------------------------------------------
pub async fn get_profile( app_state: &AppState ) -> String
{
    let client = app_state.client();

    let url = API_URL.to_string() + "/mypage/get_profile";
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    body
}
