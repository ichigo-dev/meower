//------------------------------------------------------------------------------
//! Aside.
//------------------------------------------------------------------------------

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
        li(class="clickable padding_zero")
        {
            a
            (
                href=href,
                rel="external",
                class="display_block padding_vertical_sm padding_horizontal_md",
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
pub fn Aside<G: Html>() -> View<G>
{
    view!
    {
        div(class="ui_sidebar ui_box border_right z_index_front padding_zero min_width_7xl")
        {
            nav(class="ui_sidebar_inner flex flex_column flex_gap_xl overflow_hidden width_full")
            {
                ul(class="ui_list primary simple")
                {
                    AsideNavItem(href="/", text="Home")
                    AsideNavItem(href="/account", text="Account")
                }
            }
        }
    }
}
