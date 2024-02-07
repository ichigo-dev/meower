//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextFieldVariant
{
    Default,
    Filled,
    Outlined,
}

impl TextFieldVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),
            Self::Filled => "filled".to_string(),
            Self::Outlined => "outlined".to_string(),
        }
    }
}

impl Default for TextFieldVariant
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<TextFieldVariant>> for TextFieldVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
