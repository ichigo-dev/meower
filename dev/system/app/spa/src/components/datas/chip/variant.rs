//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChipVariant
{
    Filled,
    Outlined,
}

impl ChipVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Filled => "".to_string(),
            Self::Outlined => "outlined".to_string(),
        }
    }
}

impl Default for ChipVariant
{
    fn default() -> Self
    {
        Self::Filled
    }
}

impl Into<ReadSignal<ChipVariant>> for ChipVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
