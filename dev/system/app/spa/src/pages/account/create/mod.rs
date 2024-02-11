//------------------------------------------------------------------------------
//! Account create page.
//------------------------------------------------------------------------------

mod create_account_form;

use create_account_form::CreateAccountForm;

use crate::layouts::application::Main;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Create<G: Html>() -> View<G>
{
    view!
    {
        Main(heading=t!("pages.account.create.heading"))
        {
            CreateAccountForm
        }
    }
}
