//------------------------------------------------------------------------------
//! Position.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Position.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SnackbarPosition
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

impl SnackbarPosition
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::TopLeft => "top_left".to_string(),
            Self::Top => "top".to_string(),
            Self::TopRight => "top_right".to_string(),
            Self::Left => "left".to_string(),
            Self::Right => "right".to_string(),
            Self::BottomLeft => "bottom_left".to_string(),
            Self::Bottom => "bottom".to_string(),
            Self::BottomRight => "bottom_right".to_string(),
        }
    }
}

impl Default for SnackbarPosition
{
    fn default() -> Self
    {
        Self::BottomLeft
    }
}

impl Into<ReadSignal<SnackbarPosition>> for SnackbarPosition
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
