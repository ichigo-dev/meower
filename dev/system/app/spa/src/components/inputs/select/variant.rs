//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectVariant
{
    Default,
    Standard,
    Filled,
}

impl SelectVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),
            Self::Standard => "standard".to_string(),
            Self::Filled => "filled".to_string(),
        }
    }
}

impl Default for SelectVariant
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<SelectVariant>> for SelectVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
