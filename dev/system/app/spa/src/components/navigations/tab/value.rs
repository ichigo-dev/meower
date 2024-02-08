//------------------------------------------------------------------------------
//! TabValue.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// TabValue.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct TabValue(pub &'static str);

impl Default for TabValue
{
    fn default() -> Self
    {
        Self("")
    }
}

impl Into<ReadSignal<TabValue>> for TabValue
{
    fn into(self) -> ReadSignal<TabValue>
    {
        *create_signal(self)
    }
}

impl Into<Signal<TabValue>> for TabValue
{
    fn into(self) -> Signal<TabValue>
    {
        create_signal(self)
    }
}
