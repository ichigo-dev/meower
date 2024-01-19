//------------------------------------------------------------------------------
//! Bad request page.
//------------------------------------------------------------------------------

use askama::Template;
use rust_i18n::t;


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "bad_request.html")]
pub(crate) struct PageTemplate {}
