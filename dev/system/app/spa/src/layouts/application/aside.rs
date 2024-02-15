//------------------------------------------------------------------------------
//! Aside.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// AsideNavItem.
//------------------------------------------------------------------------------
#[component(inline_props)]
fn AsideNavItem<G: Html>
(
    href: &'static str,
    text: &'static str,
) -> View<G>
{
    view!
    {
        ListItem
        (
            classes=StrProp("padding_zero").into(),
            clickable=BoolProp(true).into(),
        )
        {
            Link
            (
                href=StrProp(href).into(),
                classes=StrProp("display_block padding_vertical_sm padding_horizontal_md").into(),
            )
            {
                (text)
            }
        }
    }
}


//------------------------------------------------------------------------------
/// Aside.
//------------------------------------------------------------------------------
#[component]
pub async fn Aside<G: Html>() -> View<G>
{
    view!
    {
        Box(classes=StrProp("ui_sidebar ui_box border_right z_index_drawer padding_zero min_width_7xl").into())
        {
            Box
            (
                tag=BoxTag::Nav.into(),
                classes=StrProp("ui_sidebar_inner flex flex_column flex_gap_xl overflow_hidden width_full").into(),
            )
            {
                List
                (
                    color=Colors::Primary.into(),
                    variant=ListVariant::Simple.into(),
                )
                {
                    AsideNavItem(href="/", text="Home")
                    AsideNavItem(href="/account", text="Account")
                }
            }
        }
    }
}
