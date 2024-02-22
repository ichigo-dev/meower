//------------------------------------------------------------------------------
//! Account menu.
//------------------------------------------------------------------------------

use super::account_list::AccountList;
use crate::components::*;
use crate::utils::props::*;

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
            Box(classes=StrProp("width_8xl").into())
            {
                AccountList(open=open)
            }
        }
    }
}
