//------------------------------------------------------------------------------
//! Size.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Size.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProgressSize
{
    Small,
    Medium,
    Large,
}

impl ProgressSize
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

impl Default for ProgressSize
{
    fn default() -> Self
    {
        Self::Medium
    }
}

impl Into<ReadSignal<ProgressSize>> for ProgressSize
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
