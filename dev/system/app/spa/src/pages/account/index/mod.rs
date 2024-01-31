//------------------------------------------------------------------------------
//! Account index page.
//------------------------------------------------------------------------------

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
        div(class="flex flex_column flex_gap_lg")
        {
            h1(class="ui_heading h1 divider")
            {
                (t!("pages.account.index.heading"))
            }
            div(class="ui_box surface radius padding_lg flex flex_column flex_align_start flex_gap_md")
            {
                a
                (
                    href="/account/create",
                    rel="external",
                    class="ui_button primary"
                )
                {
                    (t!("pages.account.index.button.create_account"))
                }
                AccountList()
            }
        }
    }
}
