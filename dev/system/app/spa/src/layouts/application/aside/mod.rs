//------------------------------------------------------------------------------
//! Aside.
//------------------------------------------------------------------------------

mod account_list;
mod account_menu;
mod account_menu_button;
mod nav_item;

use account_menu::AccountMenu;
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
    let account_menu_open = create_signal(false);
    let account_menu_button_ref = create_node_ref();

    view!
    {
        Box(classes=StrProp("ui_sidebar ui_box border_right z_index_drawer padding_zero width_8xl flex_no_shrink").into())
        {
            Box
            (
                tag=BoxTag::Nav.into(),
                classes=StrProp("ui_sidebar_inner flex flex_column flex_gap_xl width_full").into(),
            )
            {
                Box(classes=StrProp("account_menu_container margin_horizontal").into())
                {
                    AccountMenu
                    (
                        anchor=account_menu_button_ref,
                        open=account_menu_open,
                    )
                    AccountMenuButton
                    (
                        node_ref=account_menu_button_ref,
                        open=account_menu_open,
                    )
                }
                List
                (
                    color=Colors::Primary.into(),
                    variant=ListVariant::Simple.into(),
                )
                {
                    NavItem(href="/", text="Home")
                }
            }
        }
    }
}
