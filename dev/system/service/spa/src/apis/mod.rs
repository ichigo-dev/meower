//------------------------------------------------------------------------------
//! APIs.
//------------------------------------------------------------------------------

pub(crate) mod mypage;

use crate::AppState;

use reqwest::{ Response, StatusCode };
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Gets response body.
//------------------------------------------------------------------------------
pub(crate) async fn get_response_body<'cx>
(
    cx: Scope<'cx>,
    response: Response,
) -> String
{
    let status = response.status();
    if status.is_success()
    {
        let body = response.text().await.unwrap_or("".to_string());
        body
    }
    else
    {
        if status == StatusCode::UNAUTHORIZED
        {
            let app_state = use_context::<AppState>(cx);
            let url = app_state.config.auth_server_url.to_owned();
            let _ = web_sys::window()
                .unwrap()
                .location()
                .set_href(&url);
        }

        "".to_string()
    }
}
