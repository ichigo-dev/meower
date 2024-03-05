//------------------------------------------------------------------------------
//! Group create page.
//------------------------------------------------------------------------------

use crate::layouts::application::{ Layout, Main };
use super::components::GroupForm;

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
            Main(heading=t!("pages.account.create_group.heading"))
            {
                GroupForm
            }
        }
    }
}
