//------------------------------------------------------------------------------
//! Signup success page.
//------------------------------------------------------------------------------

use askama::Template;
use rust_i18n::t;


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[derive(Template, Default)]
#[template(path = "signup_success.html")]
pub(crate) struct PageTemplate {}