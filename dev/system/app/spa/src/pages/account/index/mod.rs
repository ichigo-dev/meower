//------------------------------------------------------------------------------
//! Account index page.
//------------------------------------------------------------------------------

mod account_list;
use account_list::AccountList;

use crate::layouts::application::Main;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Index<G: Html>() -> View<G>
{
    view!
    {
        Main(heading=t!("pages.account.index.heading"))
        {
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
    }
}
