//------------------------------------------------------------------------------
//! Handlers for static asset contents such as CSS.
//------------------------------------------------------------------------------

use axum::response::IntoResponse;
use axum::extract::Path;
use axum::http::{ header, HeaderMap, StatusCode };

static STYLE_CSS: &str = include_str!("../../assets/css/style.css");
static SCRIPT_JS: &str = include_str!("../../assets/js/script.js");


//------------------------------------------------------------------------------
/// Handles static asset contents.
//------------------------------------------------------------------------------
pub(crate) async fn handler( Path(path): Path<String> ) -> impl IntoResponse
{
    let mut headers = HeaderMap::new();

    if path == "style.css"
    {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, STYLE_CSS)
    }
    else if path == "script.js"
    {
        headers.insert
        (
            header::CONTENT_TYPE,
            "text/javascript".parse().unwrap()
        );
        (StatusCode::OK, headers, SCRIPT_JS)
    }
    else
    {
        (StatusCode::NOT_FOUND, headers, "")
    }
}