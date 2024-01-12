//------------------------------------------------------------------------------
//! Not Found page. 
//------------------------------------------------------------------------------

use askama::Template;
use axum::response::{ Html, IntoResponse };
use rust_i18n::t;


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[derive(Template, Default)]
#[template(path = "not_found.html")]
pub(crate) struct PageTemplate {}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// Any
pub(crate) async fn handler() -> impl IntoResponse
{
    let template = PageTemplate::default();
    Html(template.render().unwrap())
}
