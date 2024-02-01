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
    cx: Scope,
    heading: String,
    child: View<G>,
) -> View<G>
{
    view!
    {
        cx,
        div(class="flex flex_justify_center")
        {
            div(class="flex flex_column flex_gap_lg max_width_breakpoint_lg width_full")
            {
                h1(class="ui_heading h1 divider") { (heading) }
                div(class="ui_box surface radius padding_lg flex flex_column flex_align_start flex_gap_md")
                {
                    (child)
                }
            }
        }
    }
}
