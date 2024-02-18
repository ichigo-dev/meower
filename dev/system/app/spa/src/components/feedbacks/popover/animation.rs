//------------------------------------------------------------------------------
//! Animation.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Animation.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PopoverAnimation
{
    Fade,
    Slide,
    Grow,
    Flash,
}

impl PopoverAnimation
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Fade => "".to_string(),
            Self::Slide => "slidein".to_string(),
            Self::Grow => "grow".to_string(),
            Self::Flash => "flash".to_string(),
        }
    }
}

impl Default for PopoverAnimation
{
    fn default() -> Self
    {
        Self::Fade
    }
}

impl Into<ReadSignal<PopoverAnimation>> for PopoverAnimation
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
