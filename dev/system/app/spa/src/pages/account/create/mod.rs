//------------------------------------------------------------------------------
//! Account create page.
//------------------------------------------------------------------------------

mod account_form;

use account_form::AccountForm;

use crate::layouts::application::{ Layout, Main };

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
        Layout
        {
            Main(heading=t!("pages.account.create.heading"))
            {
                AccountForm
            }
        }
    }
}
