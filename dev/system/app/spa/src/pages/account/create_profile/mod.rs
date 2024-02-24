//------------------------------------------------------------------------------
//! Account create page.
//------------------------------------------------------------------------------

mod account_profile_form;

use account_profile_form::AccountProfileForm;

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
                AccountProfileForm
            }
        }
    }
}
