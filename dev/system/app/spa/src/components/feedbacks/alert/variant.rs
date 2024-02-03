//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AlertVariant
{
    Default,
    Outlined,
    Filled,
}

impl AlertVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),
            Self::Outlined => "outlined".to_string(),
            Self::Filled => "filled".to_string(),
        }
    }
}

impl Default for AlertVariant
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<AlertVariant>> for AlertVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
