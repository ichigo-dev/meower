//------------------------------------------------------------------------------
//! Mypage APIs.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_schema::api::ApiResponse;
use meower_schema::api::mypage::GetProfileResponse;

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
    let client = app_state.client();
    let url = "/mypage/get_profile";
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap_or("".to_string());
    serde_json::from_str(&body).unwrap()
}
