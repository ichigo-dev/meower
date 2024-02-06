//------------------------------------------------------------------------------
//! Size.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Size.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RadioSize
{
    Small,
    Medium,
    Large,
}

impl RadioSize
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

impl Default for RadioSize
{
    fn default() -> Self
    {
        Self::Medium
    }
}

impl Into<ReadSignal<RadioSize>> for RadioSize
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
