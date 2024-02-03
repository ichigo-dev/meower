//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TableVariant
{
    Default,
    Stripe,
    StripeVertical,
    Tiled,
}

impl TableVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),
            Self::Stripe => "stripe".to_string(),
            Self::StripeVertical => "stripe_vertical".to_string(),
            Self::Tiled => "tiled".to_string(),
        }
    }
}

impl Default for TableVariant
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<TableVariant>> for TableVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
