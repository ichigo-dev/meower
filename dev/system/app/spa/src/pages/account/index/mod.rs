//------------------------------------------------------------------------------
//! Account index page.
//------------------------------------------------------------------------------

mod account_list;
use account_list::AccountList;

use crate::components::*;
use crate::layouts::application::Main;
use crate::utils::props::*;

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
            Button
            (
                href=OptionProp(Some("/account/create".to_string())).into(),
                classes=StrProp("flex_align_self_start").into(),
            )
            {
                (t!("pages.account.index.button.add_account"))
            }
            AccountList()
        }
    }
}
