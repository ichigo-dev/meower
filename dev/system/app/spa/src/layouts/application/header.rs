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
        Box(classes=StringProp("ui_box primary text_align_center padding_sm fs_2xl").into())
        {
            "Meower"
        }
    }
}
