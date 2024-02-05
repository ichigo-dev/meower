//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant
{
    Text,
    Filled,
    Outlined,
}

impl ButtonVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Text => "text".to_string(),
            Self::Filled => "".to_string(),
            Self::Outlined => "outlined".to_string(),
        }
    }
}

impl Default for ButtonVariant
{
    fn default() -> Self
    {
        Self::Filled
    }
}

impl Into<ReadSignal<ButtonVariant>> for ButtonVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
