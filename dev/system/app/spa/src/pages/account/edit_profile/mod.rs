//------------------------------------------------------------------------------
//! Account edit page.
//------------------------------------------------------------------------------

use crate::layouts::application::{ Layout, Main };
use super::components::AccountProfileForm;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn EditProfile<G: Html>() -> View<G>
{
    view!
    {
        Layout
        {
            Main(heading=t!("pages.account.edit_profile.heading"))
            {
                AccountProfileForm
            }
        }
    }
}
