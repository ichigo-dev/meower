//------------------------------------------------------------------------------
//! Group edit page.
//------------------------------------------------------------------------------

use crate::layouts::application::{ Layout, Main };
use super::components::GroupForm;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Edit<G: Html>( group_name: String ) -> View<G>
{
    view!
    {
        Layout
        {
            Main(heading=t!("pages.account.edit_group.heading"))
            {
                GroupForm
            }
        }
    }
}
