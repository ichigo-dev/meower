//------------------------------------------------------------------------------
//! Forgot password success page.
//------------------------------------------------------------------------------

use askama::Template;
use rust_i18n::t;


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "forgot_password_success.html")]
pub(crate) struct PageTemplate {}
