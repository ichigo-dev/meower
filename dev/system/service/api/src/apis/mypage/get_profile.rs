//------------------------------------------------------------------------------
//! Mypage - get profile API.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_type::JwtClaims;
use meower_schema::apis::ApiResponse;
use meower_schema::apis::mypage::GetProfileResponse;
use meower_entity::user_account::Entity as UserAccount;

use axum::response::{ Json, IntoResponse };
use axum::extract::{ State, Extension };
use rust_i18n::t;


//------------------------------------------------------------------------------
/// Gets the profile.
//------------------------------------------------------------------------------
pub async fn get_handler
(
    State(state): State<AppState>,
    Extension(jwt_claims): Extension<JwtClaims>,
) -> impl IntoResponse
{
    let uan = jwt_claims.uan;
    let user_account = match UserAccount::find_by_user_account_name(&uan)
        .one(&state.hdb)
        .await
        .unwrap()
    {
        Some(user_account) => user_account,
        None =>
        {
            let data = ApiResponse::ng(t!("apis.mypage.get_profile.not_found"));
            return Json(data);
        },
    };

    let data = ApiResponse::ok(GetProfileResponse
    {
        display_name: user_account.display_name,
    });
    Json(data)
}
