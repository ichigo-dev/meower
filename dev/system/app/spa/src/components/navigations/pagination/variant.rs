//------------------------------------------------------------------------------
//! Variant.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Variant.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PaginationVariant
{
    Circle,
    CircleOutlined,
    Rounded,
    RoundedOutlined,
}

impl PaginationVariant
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Circle => "".to_string(),
            Self::CircleOutlined => "outlined".to_string(),
            Self::Rounded => "rounded".to_string(),
            Self::RoundedOutlined => "rounded outlined".to_string(),
        }
    }
}

impl Default for PaginationVariant
{
    fn default() -> Self
    {
        Self::Circle
    }
}

impl Into<ReadSignal<PaginationVariant>> for PaginationVariant
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
