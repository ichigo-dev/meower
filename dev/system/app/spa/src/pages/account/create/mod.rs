//------------------------------------------------------------------------------
//! Account create page.
//------------------------------------------------------------------------------

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub(crate) async fn Create<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    view!
    {
        cx,
        div(class="flex flex_column flex_gap_lg")
        {
            h1(class="ui_heading h1 divider")
            {
                (t!("pages.account.create.heading"))
            }
        }
    }
}
