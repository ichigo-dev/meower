//------------------------------------------------------------------------------
//! Position.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Position.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DrawerPosition
{
    Top,
    Right,
    Bottom,
    Left,
}

impl DrawerPosition
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

impl Default for DrawerPosition
{
    fn default() -> Self
    {
        Self::Left
    }
}

impl Into<ReadSignal<DrawerPosition>> for DrawerPosition
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
