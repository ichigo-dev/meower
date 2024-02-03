//------------------------------------------------------------------------------
//! Position.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Position.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TooltipPosition
{
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipPosition
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Top => "top".to_string(),
            Self::Right => "right".to_string(),
            Self::Bottom => "bottom".to_string(),
            Self::Left => "left".to_string(),
        }
    }
}

impl Default for TooltipPosition
{
    fn default() -> Self
    {
        Self::Top
    }
}

impl Into<ReadSignal<TooltipPosition>> for TooltipPosition
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
