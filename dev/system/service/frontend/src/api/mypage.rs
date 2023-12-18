//------------------------------------------------------------------------------
//! Mypage API.
//------------------------------------------------------------------------------

use meower_api_schema::ApiResponse;
use meower_api_schema::mypage::profile::GetProfileResponse;
use crate::AppState;
use crate::api::API_URL;


//------------------------------------------------------------------------------
/// Gets profile.
//------------------------------------------------------------------------------
pub async fn get_profile
(
    app_state: &AppState,
) -> ApiResponse<GetProfileResponse>
{
    let client = app_state.client();
    let url = API_URL.to_string() + "/mypage/get_profile";
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    serde_json::from_str(&body).unwrap()
}
