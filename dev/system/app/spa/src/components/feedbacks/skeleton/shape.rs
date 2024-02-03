//------------------------------------------------------------------------------
//! Shape.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Shape.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SkeletonShape
{
    Box,
    Circle,
    Text,
}

impl SkeletonShape
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Box => "box".to_string(),
            Self::Circle => "circle".to_string(),
            Self::Text => "text".to_string(),
        }
    }
}

impl Default for SkeletonShape
{
    fn default() -> Self
    {
        Self::Box
    }
}

impl Into<ReadSignal<SkeletonShape>> for SkeletonShape
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
