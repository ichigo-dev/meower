//------------------------------------------------------------------------------
//! Footer.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Footer<G: Html>() -> View<G>
{
    view!
    {
        Box(classes=StringProp("ui_box filled text_align_center padding_sm fs_sm").into())
        {
            "© 2024 Meower."
        }
    }
}
