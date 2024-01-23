//------------------------------------------------------------------------------
//! APIs.
//------------------------------------------------------------------------------

pub(crate) mod mypage;

use reqwest::{ Response, StatusCode };


//------------------------------------------------------------------------------
/// Gets response body.
//------------------------------------------------------------------------------
pub(crate) async fn get_response_body( response: Response )-> String
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
            let _ = web_sys::window()
                .unwrap()
                .location()
                .reload();
        }

        "".to_string()
    }
}
