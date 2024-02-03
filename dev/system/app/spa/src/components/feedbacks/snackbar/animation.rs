//------------------------------------------------------------------------------
//! Animation.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Animation.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SnackbarAnimation
{
    Fade,
    Slide,
    Grow,
    Flash,
}

impl SnackbarAnimation
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

impl Default for SnackbarAnimation
{
    fn default() -> Self
    {
        Self::Fade
    }
}

impl Into<ReadSignal<SnackbarAnimation>> for SnackbarAnimation
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
