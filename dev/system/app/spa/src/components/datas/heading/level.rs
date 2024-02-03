//------------------------------------------------------------------------------
//! Level.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Level.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HeadingLevel
{
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::H1 => "h1".to_string(),
            Self::H2 => "h2".to_string(),
            Self::H3 => "h3".to_string(),
            Self::H4 => "h4".to_string(),
            Self::H5 => "h5".to_string(),
            Self::H6 => "h6".to_string(),
        }
    }
}

impl Default for HeadingLevel
{
    fn default() -> Self
    {
        Self::H3
    }
}

impl Into<ReadSignal<HeadingLevel>> for HeadingLevel
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
