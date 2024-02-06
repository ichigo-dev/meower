//------------------------------------------------------------------------------
//! Size.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Size.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RangeSliderSize
{
    Small,
    Medium,
    Large,
}

impl RangeSliderSize
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Small => "small".to_string(),
            Self::Medium => "".to_string(),
            Self::Large => "large".to_string(),
        }
    }
}

impl Default for RangeSliderSize
{
    fn default() -> Self
    {
        Self::Medium
    }
}

impl Into<ReadSignal<RangeSliderSize>> for RangeSliderSize
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
