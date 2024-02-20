//------------------------------------------------------------------------------
//! Account menu.
//------------------------------------------------------------------------------

use crate::components::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn AccountMenu<G: Html>( anchor: NodeRef<G>, open: Signal<bool> ) -> View<G>
{
    view!
    {
        Popover
        (
            anchor=anchor,
            open=open,
            position=PopoverPosition::BottomRight.into(),
        )
        {
            "account"
        }
    }
}
