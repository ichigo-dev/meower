//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonGroupVariant
{
    Text,
    Filled,
    Outlined,
}

impl ButtonGroupVariant
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

impl Default for ButtonGroupVariant
{
    fn default() -> Self
    {
        Self::Filled
    }
}

impl Into<ReadSignal<ButtonGroupVariant>> for ButtonGroupVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
