//------------------------------------------------------------------------------
//! Animation.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Animation.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SkeletonAnimation
{
    None,
    Pulse,
    Wave,
}

impl SkeletonAnimation
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::None => "".to_string(),
            Self::Pulse => "pulse".to_string(),
            Self::Wave => "wave".to_string(),
        }
    }
}

impl Default for SkeletonAnimation
{
    fn default() -> Self
    {
        Self::Wave
    }
}

impl Into<ReadSignal<SkeletonAnimation>> for SkeletonAnimation
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
