//------------------------------------------------------------------------------
//! Round.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Round.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonRound
{
    Default,
    Full,
    None,
}

impl ButtonRound
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),
            Self::Full => "full_rounded".to_string(),
            Self::None => "no_rounded".to_string(),
        }
    }
}

impl Default for ButtonRound
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<ButtonRound>> for ButtonRound
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
