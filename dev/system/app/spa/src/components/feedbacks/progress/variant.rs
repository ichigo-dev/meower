//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProgressVariant
{
    Spin,
    Linear,
}

impl ProgressVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Spin => "spin".to_string(),
            Self::Linear => "linear".to_string(),
        }
    }
}

impl Default for ProgressVariant
{
    fn default() -> Self
    {
        Self::Spin
    }
}

impl Into<ReadSignal<ProgressVariant>> for ProgressVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
