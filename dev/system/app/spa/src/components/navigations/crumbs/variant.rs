//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CrumbsVariant
{
    Default,
    Angle,
}

impl CrumbsVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),
            Self::Angle => "angle".to_string(),
        }
    }
}

impl Default for CrumbsVariant
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<CrumbsVariant>> for CrumbsVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
