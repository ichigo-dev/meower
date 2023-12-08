//------------------------------------------------------------------------------
//! Signup success page.
//------------------------------------------------------------------------------

use crate::I18n;

use askama::Template;


//------------------------------------------------------------------------------
/// Signup success page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "signup_success.html")]
pub(crate) struct SignupSuccessTemplate
{
    pub(crate) i18n: I18n,
}

impl Default for SignupSuccessTemplate
{
    fn default() -> Self
    {
        Self
        {
            i18n: I18n::new(),
        }
    }
}
