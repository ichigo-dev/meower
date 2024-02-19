//------------------------------------------------------------------------------
//! Position.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Position.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PopoverPosition
{
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Default for PopoverPosition
{
    fn default() -> Self
    {
        Self::BottomRight
    }
}

impl Into<ReadSignal<PopoverPosition>> for PopoverPosition
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
