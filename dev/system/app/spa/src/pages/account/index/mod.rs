//------------------------------------------------------------------------------
//! Account index page.
//------------------------------------------------------------------------------

use crate::layouts::application::Main;
mod account_list;
use account_list::AccountList;

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
        Main
        (
            heading=t!("pages.account.index.heading"),
            child=view!
            {
                cx,
                a
                (
                    href="/account/create",
                    rel="external",
                    class="ui_button primary",
                )
                {
                    (t!("pages.account.index.button.add_account"))
                }
                AccountList()
            }
        )
    }
}
