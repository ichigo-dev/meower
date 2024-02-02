//------------------------------------------------------------------------------
//! Main component.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Main<G: Html>
(
    heading: String,
    children: Children<G>,
) -> View<G>
{
    let children = children.call();
    view!
    {
        div(class="flex flex_justify_center")
        {
            div(class="flex flex_column flex_gap_lg max_width_breakpoint_lg width_full")
            {
                h1(class="ui_heading h1 divider") { (heading) }
                (children)
            }
        }
    }
}
