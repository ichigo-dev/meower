//------------------------------------------------------------------------------
//! Thickness.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Thickness.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HeadingThickness
{
    Thin,
    Normal,
    Thick,
}

impl HeadingThickness
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Thin => "thin".to_string(),
            Self::Normal => "".to_string(),
            Self::Thick => "thick".to_string(),
        }
    }
}

impl Default for HeadingThickness
{
    fn default() -> Self
    {
        Self::Normal
    }
}

impl Into<ReadSignal<HeadingThickness>> for HeadingThickness
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
