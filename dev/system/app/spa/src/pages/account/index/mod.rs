//------------------------------------------------------------------------------
//! Account.
//------------------------------------------------------------------------------

mod components;
use components::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub(crate) async fn Index<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    view!
    {
        cx,
        h1(class="ui_heading h1") { (t!("pages.account.index.heading")) }
        AccountList()
    }
}
