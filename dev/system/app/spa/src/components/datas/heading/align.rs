//------------------------------------------------------------------------------
//! Align.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Align.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HeadingAlign
{
    Left,
    Center,
    Right,
}

impl HeadingAlign
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Left => "left".to_string(),
            Self::Center => "center".to_string(),
            Self::Right => "right".to_string(),
        }
    }
}

impl Default for HeadingAlign
{
    fn default() -> Self
    {
        Self::Left
    }
}

impl Into<ReadSignal<HeadingAlign>> for HeadingAlign
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
