//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ListVariant
{
    Default,
    Boxed,
    Simple,
}

impl ListVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),
            Self::Boxed => "boxed".to_string(),
            Self::Simple => "simple".to_string(),
        }
    }
}

impl Default for ListVariant
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<ListVariant>> for ListVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
