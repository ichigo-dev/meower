//------------------------------------------------------------------------------
//! Size.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Size.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChipSize
{
    Small,
    Medium,
    Large,
}

impl ChipSize
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

impl Default for ChipSize
{
    fn default() -> Self
    {
        Self::Medium
    }
}

impl Into<ReadSignal<ChipSize>> for ChipSize
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
