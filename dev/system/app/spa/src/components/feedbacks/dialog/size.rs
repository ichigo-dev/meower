//------------------------------------------------------------------------------
//! Size.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Size.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DialogSize
{
    Small,
    Medium,
    Large,
    Full
}

impl DialogSize
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
            Self::Full => "full".to_string(),
        }
    }
}

impl Default for DialogSize
{
    fn default() -> Self
    {
        Self::Medium
    }
}

impl Into<ReadSignal<DialogSize>> for DialogSize
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
