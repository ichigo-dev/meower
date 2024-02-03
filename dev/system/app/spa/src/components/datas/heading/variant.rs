//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HeadingVariant
{
    Default,
    Divider,
    Bullet,
    Line,
    Band,
}

impl HeadingVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),
            Self::Divider => "divider".to_string(),
            Self::Bullet => "bullet".to_string(),
            Self::Line => "line".to_string(),
            Self::Band => "band".to_string(),
        }
    }
}

impl Default for HeadingVariant
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<HeadingVariant>> for HeadingVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
