//------------------------------------------------------------------------------
//! Header.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Header<G: Html>() -> View<G>
{
    view!
    {
        Box(classes=StrProp("ui_box primary color_primary_text text_align_center padding_sm fs_2xl").into())
        {
            "Meower"
        }
    }
}
