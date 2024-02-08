//------------------------------------------------------------------------------
//! Panel component.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn MainPanel<G: Html>( children: Children<G> ) -> View<G>
{
    let children = children.call();
    view!
    {
        Box(classes=StrProp("ui_box surface radius padding_lg flex flex_column flex_align_start flex_gap_md overflow_auto_x").into())
        {
            (children)
        }
    }
}
