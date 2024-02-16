//------------------------------------------------------------------------------
//! Aside.
//------------------------------------------------------------------------------

mod account_list;
mod account_menu_button;
mod nav_item;

use account_list::AccountList;
use account_menu_button::AccountMenuButton;
use nav_item::NavItem;

use crate::components::*;
use crate::utils::props::*;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Aside.
//------------------------------------------------------------------------------
#[component]
pub fn Aside<G: Html>() -> View<G>
{
    view!
    {
        Box(classes=StrProp("ui_sidebar ui_box border_right z_index_drawer padding_zero width_7xl overflow_hidden").into())
        {
            Box
            (
                tag=BoxTag::Nav.into(),
                classes=StrProp("ui_sidebar_inner flex flex_column flex_gap_xl overflow_hidden width_full").into(),
            )
            {
                Box(classes=StrProp("margin_horizontal_md").into())
                {
                    AccountMenuButton
                }
                List
                (
                    color=Colors::Primary.into(),
                    variant=ListVariant::Simple.into(),
                )
                {
                    NavItem(href="/", text="Home")
                    NavItem(href="/account", text="Account")
                }
            }
        }
    }
}
