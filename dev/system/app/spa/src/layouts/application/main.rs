//------------------------------------------------------------------------------
//! Main component.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;

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
        Box(classes=StrProp("flex flex_justify_center").into())
        {
            Box(classes=StrProp("flex flex_column flex_gap_lg max_width_breakpoint_lg width_full overflow_hidden").into())
            {
                Heading
                (
                    level=HeadingLevel::H2.into(),
                    variant=HeadingVariant::Divider.into(),
                )
                {
                    (heading)
                }
                (children)
            }
        }
    }
}
