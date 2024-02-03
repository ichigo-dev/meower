//------------------------------------------------------------------------------
//! Animation.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Animation.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DialogAnimation
{
    Fade,
    Slide,
    Grow,
    Flash,
}

impl DialogAnimation
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

impl Default for DialogAnimation
{
    fn default() -> Self
    {
        Self::Fade
    }
}

impl Into<ReadSignal<DialogAnimation>> for DialogAnimation
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
