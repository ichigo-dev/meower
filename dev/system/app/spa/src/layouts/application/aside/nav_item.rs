//------------------------------------------------------------------------------
//! Aside navigation item.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// AsideNavItem.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn NavItem<G: Html>
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
