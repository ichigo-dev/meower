//------------------------------------------------------------------------------
//! Mypage APIs.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_schema::apis::ApiResponse;
use meower_schema::apis::mypage::GetProfileResponse;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Gets profile.
//------------------------------------------------------------------------------
pub async fn get_profile<'cx>
(
    cx: Scope<'cx>,
) -> ApiResponse<GetProfileResponse>
{
    let app_state = use_context::<AppState>(cx);
    let url = app_state.config.api_url.to_owned() + "/mypage/get_profile";
    let response = app_state.client
        .get(url)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    serde_json::from_str(&body).unwrap()
}
