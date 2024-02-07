//------------------------------------------------------------------------------
//! Account create page.
//------------------------------------------------------------------------------

use crate::layouts::application::Main;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Create<G: Html>() -> View<G>
{
    view!
    {
        Main(heading=t!("pages.account.create.heading"))
        {
            form(class="flex flex_column flex_gap_md width_full")
            {
            }
        }
    }
}
