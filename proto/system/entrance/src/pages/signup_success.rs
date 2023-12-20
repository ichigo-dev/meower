//------------------------------------------------------------------------------
//! Signup success page.
//------------------------------------------------------------------------------

use crate::I18n;

use askama::Template;


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "signup_success.html")]
pub(crate) struct SignupSuccessTemplate
{
    pub(crate) i18n: I18n,
}
